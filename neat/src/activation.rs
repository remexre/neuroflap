use std::cmp::max;

use float_ord::FloatOrd;

/// The activation function for each neuron.
#[derive(Clone, Copy, Debug, Display, EnumString, PartialEq, PartialOrd)]
pub enum Activation {
    /// max(0, x)
    ReLU,

    /// 2 / (1 + e ^ (-4.9*x)) - 1
    Sigmoid,

    /// tanh(x)
    Tanh,
}

impl Activation {
    /// Calculates the activation function for the given value.
    pub fn calculate(self, x: f32) -> f32 {
        match self {
            Activation::ReLU => max(FloatOrd(x), FloatOrd(0.0)).0,
            Activation::Sigmoid => (2.0 / (1.0 + (-4.9 * x).exp())) - 1.0,
            Activation::Tanh => x.tanh(),
        }
    }
}

impl Default for Activation {
    fn default() -> Activation {
        Activation::Sigmoid
    }
}
