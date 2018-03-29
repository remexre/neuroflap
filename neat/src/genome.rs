use std::collections::HashSet;
use std::iter::FromIterator;

/// The entire genome of an organism.
///
/// Note that all genomes have the same nodes 0-5, with 1-5 being inputs and 0
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

        let froms: HashSet<usize> = HashSet::from_iter(1..=5);
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
