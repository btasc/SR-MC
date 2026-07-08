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

    #[inline]
    pub fn in_bounds(&self, coords: [usize; N]) -> bool {
        (0..N).all(|i| coords[i] < self.dims[i])
    }

    pub fn get(&self, coords: [usize; N]) -> T {
        debug_assert!(self.in_bounds(coords), "Volume::get was given indices out of bounds. Coords: {:?}. Dimensions: {:?}", coords, self.dims);
        self.data[self.index(coords)]
    }

    pub fn get_mut(&mut self, coords: [usize; N]) -> &mut T {
        debug_assert!(self.in_bounds(coords), "Volume::get_mut was given indices out of bounds. Coords: {:?}. Dimensions: {:?}", coords, self.dims);
        &mut self.data[self.index(coords)]
    }
}

pub struct Material {
    pub mu_a: f32,
    pub mu_s: f32,
    pub mu_t: f32,
    
    // add henyey greenstein values
    pub g: f32,
    pub g_2: f32,
    pub inv_2g: f32,
}

pub struct PhotonStateF32 {
    // position 0 through 1 inside the current voxel
    pub pos: Vec3,
    // global voxel coordinate
    pub global_pos: [usize; 3],
    pub dir: Vec3,
    pub weight: f32,
    pub path: f32,
    pub is_path_done: bool,
    pub time: f32,
    pub voxel_acc: f32,
    pub current_voxel_idx: usize,
    pub rng: rand::rngs::ThreadRng,
}