use tch::{
    self,
    nn::{self, LinearConfig, Module, OptimizerConfig, Sequential},
    Tensor,
};

const INPUT_DIM: i64 = 363;
const HIDDEN_LAYER: i64 = INPUT_DIM * 2;
const OUTPUT_DIM: i64 = 1;

pub struct Network {
    net: Sequential,
}

impl Network {
    pub fn new(vs: &nn::VarStore) -> Self {
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
                OUTPUT_DIM,
                Default::default(),
            ))
            .add_fn(|xs| xs.tanh());

        Self { net }
    }

    pub fn forward(&self, xs: &Tensor) -> Tensor {
        self.net.forward(&xs.flat_view())
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

        vs.save("model.ot").unwrap()
    }
}
