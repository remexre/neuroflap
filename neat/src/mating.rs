use rand::Rng;

use genome::Genome;
use params::Params;
use species::Species;

/// The "classification" of a gene.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum GeneClass {
    /// The gene appears on both genes. The parameter is the absolute
    /// difference between the two.
    Matching(f32),

    /// The gene appears only the left organism, and evolved more recently than
    /// any of the other's genes.
    ExcessLeft,

    /// The gene appears only the right organism, and evolved more recently
    /// than any of the other's genes.
    ExcessRight,

    /// The gene appears only on the left organism, and is not an excess gene.
    DisjointLeft,

    /// The gene appears only on the right organism, and is not an excess gene.
    DisjointRight,
}

/// Classifies the genes of each of two organisms.
pub fn classify_genes(g1: &Genome, g2: &Genome) -> Vec<GeneClass> {
    unimplemented!()
}

/// Computes the difference between two organisms.
pub fn difference(g1: &Genome, g2: &Genome, params: &Params) -> f32 {
    let mut e = 0.0;
    let mut d = 0.0;
    let mut w_sum = 0.0;

    let classes = classify_genes(g1, g2);
    let n = classes.len() as f32;
    for gene in classes {
        match gene {
            GeneClass::Matching(d) => w_sum += d,
            GeneClass::ExcessLeft | GeneClass::ExcessRight => e += 1.0,
            GeneClass::DisjointLeft | GeneClass::DisjointRight => d += 1.0,
        }
    }
    let w = w_sum / n;

    let n = if g1.len() < 20 && g2.len() < 20 {
        1.0
    } else {
        n
    };

    e /= n;
    d /= n;

    params.c1 * e + params.c2 * d + params.c3 * w
}

/// Classifies new organisms into species based on the previous species.
///
/// Note that this will panic if any empty species is provided. Don't do that.
pub fn classify_species<I, R>(
    rng: &mut R,
    organisms: I,
    params: &Params,
    previous_species: &[Species],
) -> Vec<Species>
where
    I: IntoIterator<Item = Genome>,
    R: Rng,
{
    let exemplars = previous_species
        .iter()
        .map(|species| rng.choose(&species.0).unwrap())
        .collect::<Vec<_>>();
    let out = vec![Species::empty(); previous_species.len()];
    unimplemented!()
}
