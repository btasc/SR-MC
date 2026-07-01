use rand::RngExt;
use crate::config::*;

pub fn resolve_photon(photon: PhotonStateF32, volume_in: &Volume<u8, 3>, fluence_out: &mut Volume<f32, 4>, materials: &[Material]) {
    // confirm that the inner volumes are the same shape to avoid indexing errors
    debug_assert_eq!(volume_in.dims[0], fluence_out.dims[0]);
    debug_assert_eq!(volume_in.dims[1], fluence_out.dims[1]);
    debug_assert_eq!(volume_in.dims[2], fluence_out.dims[2]);

    let mut rng = rand::rng();
    let mut photon = photon;

    // sample photon path
    {
        // make sure that there isn't a path left to be traveled in debug
        // to avoid float errors, it should be set to 0.0 once done to make sure
        debug_assert_eq!(photon.path, 0.0);
        debug_assert_eq!(photon.is_path_done, true);

        photon.path = -(1.0 - rng.random::<f32>()).ln() / materials[photon.current_voxel_idx].mu_t;
    }
}