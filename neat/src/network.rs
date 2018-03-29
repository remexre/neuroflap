use genome::Genome;

/// The representation of a neural network.
#[derive(Debug)]
pub struct Network {
    neurons: Vec<Neuron>,
}

impl Genome {
    /// Builds a network from the genome.
    pub fn build_network(&self) -> Network {
        let mut genes = self.genes.clone();
        genes.sort_by_key(|gene| gene.to);

        let to_max = genes
            .iter()
            .map(|gene| gene.to)
            .max()
            .unwrap_or(0);
        let mut neurons = vec![Neuron::default(); to_max + 1];

        unimplemented!()
    }
}

impl Network {
    /// Calculates the output value of the network for a given input vector.
    pub fn calculate(ins: [f32; 5]) -> f32 {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Default)]
struct Neuron {
    value: f32,
}
