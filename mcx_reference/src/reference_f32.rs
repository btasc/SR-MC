use rand::RngExt;
use glam::Vec3;
use crate::config::*;

pub fn resolve_photon(photon: PhotonStateF32, volume_in: &Volume<u8, 3>, fluence_out: &mut Volume<f32, 4>, materials: &[Material]) {
    // confirm that the inner volumes are the same shape to avoid indexing errors
    debug_assert_eq!(volume_in.dims[0], fluence_out.dims[0]);
    debug_assert_eq!(volume_in.dims[1], fluence_out.dims[1]);
    debug_assert_eq!(volume_in.dims[2], fluence_out.dims[2]);

    let mut rng = rand::rng();
    let mut photon = photon;
}

// helpers

fn voxel_t_exit(dir: Vec3, pos: Vec3) -> f32 {
    let inv_dir = dir.recip(); // 1/dir per component

    let tlo = (Vec3::ZERO - pos) * inv_dir;
    let thi = (Vec3::ONE - pos) * inv_dir;

    let t_exits = tlo.max(thi);
    // first exit to occur
    t_exits.min_element()
}

// main kernels

fn sample_path(photon: &mut PhotonStateF32) {
    // make sure that there isn't a path left to be traveled in debug
    // to avoid float errors, it should be set to 0.0 once done to make sure
    debug_assert_eq!(photon.path, 0.0);
    debug_assert_eq!(photon.is_path_done, true);

    // Subtract from one to avoid the ln of 0
    photon.path = -(1.0 - photon.rng.random::<f32>()).ln();
}

fn traverse(photon: &mut PhotonStateF32, volume_in: &Volume<u8, 3>, materials: &[Material]) {
    debug_assert_ne!(photon.path, 0.0);
    debug_assert_eq!(photon.is_path_done, false);

    let mat = &materials[photon.current_voxel_idx];

    let len = voxel_t_exit(photon.dir, photon.pos);
    let slen = len * mat.mu_s;

    if slen <= photon.path {
        // case that photon crosses voxel boundary
        photon.path -= slen;
        photon.pos += photon.dir * len;
        // update voxel idx with new voxel
        photon.current_voxel_idx = volume_in.get(photon.global_pos) as usize;
    } else {
        // case that photon finishes path in voxel
        photon.pos += photon.dir * (photon.path / mat.mu_s);
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