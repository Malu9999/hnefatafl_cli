use tch::{
    nn::{Sequential, VarStore},
    Device,
};

use crate::{
    game::{
        board::{Board, GameState},
        piece::PieceColor,
        r#move::Move,
    },
    synthesis::network::Network,
};

use super::{Eval, EvalInit};

pub struct NeuralNet {
    net: Network,
}

impl EvalInit for NeuralNet {
    type Param = String;

    fn new(param: Self::Param) -> Self {
        let mut nn = Network::new();

        nn.load(&param);

        NeuralNet { net: nn }
    }
}

impl Eval for NeuralNet {
    fn get_eval(&self, board: &Board) -> f64 {
        f64::try_from(
            self.net.forward(
                &board
                    .get_observation()
                    .unsqueeze(0)
                    .to_device(Device::cuda_if_available()),
            ),
        )
        .unwrap()
    }

    fn update(&mut self, board: Board) {
        ()
    }
}
