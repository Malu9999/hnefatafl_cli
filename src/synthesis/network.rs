use std::path;

use tch::{
    self,
    nn::{self, LinearConfig, Module, Optimizer, OptimizerConfig, Sequential, VarStore},
    Device, Tensor,
};

const INPUT_DIM: i64 = 484;
const HIDDEN_LAYER: i64 = INPUT_DIM * 2;
const OUTPUT_DIM: i64 = 1;

pub struct Network {
    net: Sequential,
    vs: VarStore,
    opt: Optimizer,
}

impl Network {
    pub fn new() -> Self {
        let vs = VarStore::new(Device::cuda_if_available());

        let root = &vs.root();

        let net = nn::seq()
            .add(nn::linear(
                root / "layer_1",
                INPUT_DIM,
                HIDDEN_LAYER,
                Default::default(),
            ))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(
                root / "layer_2",
                HIDDEN_LAYER,
                HIDDEN_LAYER,
                Default::default(),
            ))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(
                root / "layer_3",
                HIDDEN_LAYER,
                OUTPUT_DIM,
                Default::default(),
            ))
            .add_fn(|xs| xs.tanh());

        let opt = nn::Adam::default().build(&vs, 1e-4).unwrap();

        Self { net, vs, opt }
    }

    pub fn load(&mut self, path: &str) {
        self.vs.load(path);
    }

    pub fn save(&self, path: &str) {
        self.vs.save(path).unwrap()
    }

    pub fn forward(&self, xs: &Tensor) -> Tensor {
        self.net.forward(&xs.flat_view())
    }

    pub fn train(&mut self, data: Tensor, targets: Tensor, max_epochs: usize) {
        for epoch in 0..max_epochs {
            let logits = self.forward(&data);

            let loss = logits.mse_loss(&targets, tch::Reduction::Mean);

            self.opt.zero_grad();
            loss.backward();
            self.opt.step();

            if (epoch + 1) % 10 == 0 {
                println!("Epoch: {:3}, Loss: {:?}", epoch + 1, f32::try_from(loss));
            }
        }

        self.save("checkpoint.ot");
    }
}
