mod mutate;

use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter::FromIterator;

use rand::Rng;

use params::Params;

/// The entire genome of an organism.
///
/// Note that all genomes have the same nodes 0-4, with 1-4 being inputs and 0
/// being the output. Other nodes are inferred by being referenced.
#[derive(Clone, Debug, Deserialize, Index, PartialEq, Serialize)]
pub struct Genome {
    /// The connection genes.
    pub genes: Vec<Gene>,
}

impl Genome {
    /// Creates a new, empty genome.
    pub fn new() -> Genome {
        Genome {
            genes: Vec::new(),
        }
    }

    /// Returns the number of genes in the genome.
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    /// Performs a random mutation.
    pub fn mutate<I: FnMut() -> usize, R: Rng>(
        &mut self,
        r: &mut R,
        inno: I,
        params: &Params,
    ) {
        match r.gen_range(0, 3) {
            0 => self.mutate_add_connection(r, inno),
            1 => self.mutate_add_node(r, inno),
            2 => self.mutate_reweight(r, params),
            _ => unreachable!(),
        }
    }

    /// Returns whether this represents a valid genome.
    ///
    /// This is a somewhat expensive operation.
    pub fn validate(&self) -> bool {
        let froms: HashSet<usize> = self.genes.iter().map(|c| c.from).collect();
        let tos: HashSet<usize> = self.genes.iter().map(|c| c.to).collect();

        let singleton_from: HashSet<usize> =
            froms.difference(&tos).cloned().collect();
        let singleton_to: HashSet<usize> =
            tos.difference(&froms).cloned().collect();

        let froms: HashSet<usize> = HashSet::from_iter(1..=4);
        let tos: HashSet<usize> = HashSet::from_iter(Some(0));

        if singleton_from != froms || singleton_to != tos {
            return false;
        }

        let mut last = 0;
        for gene in &self.genes {
            if gene.innovation <= last {
                return false;
            }
            last = gene.innovation;
        }
        true
    }
}

impl Display for Genome {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "[")?;

        let mut first = true;
        for gene in &self.genes {
            if first {
                first = false;
            } else {
                write!(
                    fmt,
                    "; {},{},{},{},{}",
                    gene.from,
                    gene.to,
                    gene.enabled,
                    gene.weight,
                    gene.innovation
                )?;
            }
        }

        write!(fmt, "]")
    }
}

/// A gene that represents a link between nodes in the neural net.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Gene {
    /// The input node.
    pub from: usize,

    /// The output node.
    pub to: usize,

    /// Whether the gene is enabled or not.
    pub enabled: bool,

    /// The connection weight.
    pub weight: f32,

    /// The innovation number, which is always greater than zero.
    pub innovation: usize,
}
