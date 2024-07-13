use tch::{
    self,
    nn::{self, LinearConfig, OptimizerConfig},
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
        xs.flat_view()
            .apply(&self.l_1)
            .relu()
            .apply(&self.l_2)
            .tanh()
    }

    pub fn train(&self, data: Tensor, targets: Tensor, max_epochs: usize, vs: &nn::VarStore) {
        let mut opt = nn::Adam::default().build(vs, 1e-3).unwrap();

        for epoch in 0..max_epochs {
            let logits = self.forward(&data);

            let loss = logits.mse_loss(&targets, tch::Reduction::Mean);

            opt.zero_grad();
            loss.backward();
            opt.step();

            if epoch % 10 == 0 {
                println!("Epoch: {:3}, Loss: {:?}", epoch, f32::try_from(loss));
            }
        }
    }
}
