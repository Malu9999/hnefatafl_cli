use tch::{
    self,
    nn::{self, LinearConfig},
    Tensor,
};

pub struct Network {
    l_1: nn::Linear,
    l_2: nn::Linear,
}

impl Network {
    pub fn new(vs: &nn::VarStore) -> Self {
        let root = &vs.root();

        Self {
            l_1: nn::linear(root / "l_1", 363, 726, Default::default()),
            l_2: nn::linear(root / "l_2", 726, 1, Default::default()),
        }
    }

    pub fn forward(&self, xs: &Tensor) -> Tensor {
        xs.flat_view().apply(&self.l_1).relu().apply(&self.l_2)
    }
}
