use tch::{
    self,
    nn::{self, Module, Optimizer, OptimizerConfig, Sequential, VarStore},
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
    /// creates a new network
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

    /// loads the network from a checkpoint
    pub fn load(&mut self, path: &str) {
        let _ = self.vs.load(path);
    }

    /// saves the network
    pub fn save(&self, path: &str) {
        self.vs.save(path).unwrap()
    }

    /// performs a forward pass
    pub fn forward(&self, xs: &Tensor) -> Tensor {
        self.net.forward(&xs.flat_view())
    }

    /// trains the network
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
