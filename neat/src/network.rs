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

impl Network {
    /// Calculates the output value of the network for a given input vector.
    pub fn calculate(&self, ins: [f32; 4]) -> f32 {
        let mut values = vec![None; self.neurons.len()];
        for (i, &x) in ins.iter().enumerate() {
            values[i + 1] = Some(x);
        }

        fn search(
            n: usize,
            activation: Activation,
            neurons: &[Neuron],
            values: &mut [Option<f32>],
        ) {
            if values[n].is_some() {
                return;
            }

            let neuron = &neurons[n];
            for &(i, _) in &neuron.incoming {
                search(i, activation, neurons, values);
            }

            let v = neuron
                .incoming
                .iter()
                .map(|&(i, w)| values[i].unwrap() * w)
                .sum();
            values[n] = Some(v);
        }

        search(0, self.activation, &self.neurons, &mut values);
        values[0].unwrap_or(0.0)
    }
}

#[derive(Clone, Debug, Default)]
struct Neuron {
    incoming: Vec<(usize, f32)>,
}
