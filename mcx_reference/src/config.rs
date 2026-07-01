use glam::Vec3;

// create a general volume struct that can handle multiple dimensions
// this is for the fluence output, which has a 4th 't' dimension of time
pub struct Volume<T, const N: usize> {
    pub dims: [usize; N],
    // uses a box to avoid potential vec allocation
    data: Box<[T]>,
}

impl<T: Copy + Default, const N: usize> Volume<T, N> {
    pub fn new(dims: [usize; N]) -> Self {
        let total: usize = dims.iter().product();
        Self { dims, data: vec![T::default(); total].into_boxed_slice() }
    }

    #[inline]
    fn index(&self, coords: [usize; N]) -> usize {
        let mut idx = 0;
        let mut stride = 1;

        for i in 0..N {
            idx += coords[i] * stride;
            stride *= self.dims[i];
        }

        idx
    }

    pub fn get(&self, coords: [usize; N]) -> T {
        self.data[self.index(coords)]
    }

    pub fn get_mut(&mut self, coords: [usize; N]) -> &mut T {
        &mut self.data[self.index(coords)]
    }
}

pub struct Material {
    // unimplemented for now, I plan to add fields as I write the relevant kernels
}

pub struct PhotonStateF32 {
    local_pos: Vec3,
    global_pos: [usize; 3],
    weight: f32,
    path: f32,
    time: f32,
    voxel_acc: f32,
}