use std::process::exit;
use rand::RngExt;
use glam::{Vec3, USizeVec3};
use crate::config::*;

pub fn resolve_photon(photon: PhotonStateF32, volume_in: &Volume<u8, 3>, fluence_out: &mut Volume<f32, 4>, materials: &[Material]) {
    // confirm that the inner volumes are the same shape to avoid indexing errors
    debug_assert_eq!(volume_in.dims[0], fluence_out.dims[0]);
    debug_assert_eq!(volume_in.dims[1], fluence_out.dims[1]);
    debug_assert_eq!(volume_in.dims[2], fluence_out.dims[2]);

    let mut org_photon = photon;
    let photon = &mut org_photon;

    // start main loop
    loop {
        sample_path(photon);

        while !photon.is_path_done {
            traverse(photon, volume_in, fluence_out, materials);
            if photon.is_terminated { break; }
        }

        roulette(photon);

        if photon.is_terminated {
            // flush out before terminating
            *fluence_out.get_mut(get_out_gate(photon)) += photon.voxel_acc;
            break;
        }

        scatter(photon, materials);
    }
}

// helpers

fn voxel_t_exit(dir: Vec3, pos: Vec3) -> (usize, f32) /* (axis, t) */ {
    // not needed in final implementation
    // fixes the issue of just using .trunc on pos, as if pos should be 1.0, trunc would make it 0.0
    let ;

    let inv_dir = dir.recip();

    let tlo = (Vec3::ZERO - pos) * inv_dir;
    let thi = (Vec3::ONE - pos) * inv_dir;

    let t_exits = tlo.max(thi);

    // returns t_exit and its axis
    t_exits
        .to_array().into_iter().enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap()
}

// main kernels

const THRESH: f32 = 1e-4;
const M: f32 = 10.0;

fn roulette(photon: &mut PhotonStateF32) {
    debug_assert!(M > 1.0);

    if photon.weight >= THRESH {
        return
    }

    if photon.rng.random::<f32>() < 1.0 / M {
        photon.weight *= M;
    } else {
        photon.is_terminated = true;
    }
}

fn sample_path(photon: &mut PhotonStateF32) {
    // make sure that there isn't a path left to be traveled in debug
    // to avoid float errors, it should be set to 0.0 once done to make sure
    debug_assert_eq!(photon.path, 0.0);
    debug_assert_eq!(photon.is_path_done, true);

    // Subtract from one to avoid the ln of 0
    photon.path = -(1.0 - photon.rng.random::<f32>()).ln();
    photon.is_path_done = false;
}

// temp function for now
fn get_out_gate(photon: &PhotonStateF32) -> [usize; 4]  {
    let [x, y, z] = photon.global_pos;
    // will eventually calculate the correct gate given, for now just uses 0
    let g: usize = 0;

    [x, y, z, g]
}

fn traverse(photon: &mut PhotonStateF32, volume_in: &Volume<u8, 3>, fluence_out: &mut Volume<f32, 4>, materials: &[Material]) {
    // in the edge case that sample_path draws a path with length 0, this could trip, although its extremely unlikely
    debug_assert_ne!(photon.path, 0.0);
    debug_assert_eq!(photon.is_path_done, false);

    let mat = &materials[photon.current_voxel_idx];

    let global_vec3 = (USizeVec3::from_array(photon.global_pos)).as_vec3();
    let updated_pos = photon.pos - global_vec3;

    let (exit_axis, t_exit) = voxel_t_exit(photon.dir, updated_pos);
    let slen = t_exit * mat.mu_s;

    let did_cross_bound = slen <= photon.path;
    let traveled = if did_cross_bound { t_exit } else { photon.path / mat.mu_s };

    photon.pos += photon.dir * traveled;
    photon.time += traveled * mat.n;

    // fraction of kept weight, not actual weight deposited
    let kept = (-mat.mu_a * traveled).exp();
    photon.voxel_acc += photon.weight * (1.0 - kept);
    photon.weight *= kept;

    if did_cross_bound {
        photon.path -= slen;

        // check if the photon moves exactly to the border, if so stop it but also write out voxel_acc
        if photon.path <= 0.0 {
            photon.path = 0.0;
            photon.is_path_done = true;
        }

        *fluence_out.get_mut(get_out_gate(photon)) += photon.voxel_acc;
        photon.voxel_acc = 0.0;

        let is_dir_pos = photon.dir[exit_axis] > 0.0;

        let is_overflow = is_dir_pos && photon.global_pos[exit_axis] == volume_in.dims[exit_axis] - 1;
        let is_underflow = !is_dir_pos && photon.global_pos[exit_axis] == 0;

        if is_overflow || is_underflow {
            photon.is_terminated = true;
            return;
        }

        if is_dir_pos {
            photon.global_pos[exit_axis] += 1;
        } else {
            photon.global_pos[exit_axis] -= 1;
        }

        photon.current_voxel_idx = volume_in.get(photon.global_pos) as usize;
    } else {
        photon.path = 0.0;
        photon.is_path_done = true;
    }
}

fn scatter(photon: &mut PhotonStateF32, materials: &[Material]) {
    /*
        whole equation is (1/2g) * ( 1 + g^2 - ( ( 1 - g^2 ) / ( 1 - g + 2ge ) )^2 )
        1/2g can be mat.inv_2g, g^2 can be mat.g_2, g can be mat.g
    */

    let eps = photon.rng.random::<f32>();
    let mat = &materials[photon.current_voxel_idx];

    let cos_theta = if mat.g.abs() < 1e-4 {
        2.0 * eps - 1.0
    } else {
        let inner = (1.0 - mat.g_2) / (1.0 - mat.g + 2.0 * mat.g * eps);
        (1.0 + mat.g_2 - inner * inner) * mat.inv_2g
    };

    let cos_theta = cos_theta.clamp(-1.0, 1.0);

    let azi = photon.rng.random::<f32>() * std::f32::consts::TAU;
    let (azi_sin, azi_cos) = azi.sin_cos();

    let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
    let dir = photon.dir;

    photon.dir = if dir.z.abs() > 0.99999 {
        let s = dir.z.signum();
        Vec3::new(sin_theta * azi_cos, sin_theta * azi_sin, cos_theta * s)
            .normalize()
    } else {
        let denom = (1.0 - dir.z * dir.z).sqrt();

        let new_x = sin_theta * (dir.x * dir.z * azi_cos - dir.y * azi_sin) / denom + dir.x * cos_theta;
        let new_y = sin_theta * (dir.y * dir.z * azi_cos + dir.x * azi_sin) / denom + dir.y * cos_theta;
        let new_z = -sin_theta * azi_cos * denom + dir.z * cos_theta;

        Vec3::new(new_x, new_y, new_z).normalize()
    };
}