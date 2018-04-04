use std::cmp::max;

use activation::Activation;
use genome::Genome;

/// The representation of a neural network.
#[derive(Debug)]
pub struct Network {
    activation: Activation,
    neurons: Vec<Neuron>,
}

impl Genome {
    /// Builds a network from the genome, using the given activation function.
    pub fn build_network(&self, activation: Activation) -> Network {
        let mut genes = self.genes.clone();
        genes.sort_by_key(|gene| gene.to);

        let iter = genes.iter().filter(|gene| gene.enabled);
        let to_max = iter.clone()
            .map(|gene| gene.to)
            .max()
            .unwrap_or(0);

        let mut neurons = vec![Neuron::default(); max(to_max + 1, 5)];
        for gene in iter {
            neurons[gene.to]
                .incoming
                .push((gene.from, gene.weight));
        }

        Network {
            activation,
            neurons,
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    White,
    Gray,
    Black(f32),
}

impl State {
    fn to_option(self) -> Option<f32> {
        match self {
            State::Black(x) => Some(x),
            _ => None,
        }
    }
}

impl Network {
    /// Calculates the output value of the network for a given input vector.
    pub fn calculate(&self, ins: [f32; 4]) -> f32 {
        let mut values = vec![State::White; self.neurons.len()];
        for (i, &x) in ins.iter().enumerate() {
            values[i + 1] = State::Black(x);
        }

        fn search(
            n: usize,
            activation: Activation,
            neurons: &[Neuron],
            values: &mut [State],
        ) {
            match values[n] {
                State::Black(_) => {
                    return;
                }
                State::Gray => {
                    println!("self-edge?");
                    return;
                }
                State::White => {
                    values[n] = State::Gray;
                }
            }

            let neuron = &neurons[n];
            for &(i, _) in &neuron.incoming {
                search(i, activation, neurons, values);
            }

            let v = neuron
                .incoming
                .iter()
                .map(|&(i, w)| values[i].to_option().unwrap() * w)
                .sum();
            values[n] = State::Black(v);
        }

        search(0, self.activation, &self.neurons, &mut values);
        values[0].to_option().unwrap_or(0.0)
    }
}

#[derive(Clone, Debug, Default)]
struct Neuron {
    incoming: Vec<(usize, f32)>,
}
