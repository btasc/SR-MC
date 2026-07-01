use crate::config::*;

pub fn resolve_photon(photon: PhotonStateF32, volume_in: &Volume<u8, 3>, fluence_out: &mut Volume<f32, 4>) {
    // confirm that the inner volumes are the same shape to avoid indexing errors
    debug_assert_eq!(volume_in.dims[0], fluence_out.dims[0]);
    debug_assert_eq!(volume_in.dims[1], fluence_out.dims[1]);
    debug_assert_eq!(volume_in.dims[2], fluence_out.dims[2]);

    
}