use std::cmp::max;

use rand::Rng;

use genome::{Gene, Genome};
use params::Params;

impl Genome {
    /// Adds a random connection.
    pub fn mutate_add_connection<I: FnMut() -> usize, R: Rng>(
        &mut self,
        r: &mut R,
        mut inno: I,
    ) {
        let max_neuron = max(
            5,
            self.genes
                .iter()
                .map(|c| c.to)
                .max()
                .unwrap_or(0),
        );

        let gene = loop {
            let from = r.gen_range(0, max_neuron);
            let to = r.gen_range(0, max_neuron);
            if from == to {
                continue;
            }

            if self.genes.iter().any(|g| {
                (g.from == from && g.to == to) || (g.from == to && g.to == from)
            }) {
                continue;
            }

            break Gene {
                from,
                to,
                enabled: true,
                weight: r.gen(),
                innovation: inno(),
            };
        };

        self.genes.push(gene);
    }

    /// Splits a connection to add a node. If there are no connections, this
    /// is a no-op.
    pub fn mutate_add_node<I: FnMut() -> usize, R: Rng>(
        &mut self,
        r: &mut R,
        mut inno: I,
    ) {
        // Return if there are no connections.
        if self.genes
            .iter()
            .filter(|g| g.enabled)
            .next()
            .is_none()
        {
            return;
        }

        // The number of neurons.
        let max_neuron = max(
            5,
            self.genes
                .iter()
                .map(|c| c.to)
                .max()
                .unwrap_or(0),
        );

        // The edge to split.
        let i = loop {
            let i = r.gen_range(0, self.genes.len());
            if !self.genes[i].enabled {
                continue;
            }

            break i;
        };

        // Split the edge.
        self.genes[i].enabled = false;
        let g1 = Gene {
            from: self.genes[i].from,
            to: max_neuron,
            enabled: true,
            weight: 1.0,
            innovation: inno(),
        };
        let g2 = Gene {
            from: max_neuron,
            to: self.genes[i].to,
            enabled: true,
            weight: self.genes[i].weight,
            innovation: inno(),
        };
        self.genes.push(g1);
        self.genes.push(g2);
    }

    /// Modifies connection weights.
    pub fn mutate_reweight<R: Rng>(&mut self, r: &mut R, params: &Params) {
        for gene in self.genes.iter_mut() {
            if r.next_f32() < params.reweight_rate {
                gene.weight += r.gen_range(
                    -params.reweight_amount,
                    params.reweight_amount,
                );
            }
        }
    }
}
