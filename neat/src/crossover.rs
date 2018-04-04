use std::cmp::max;

use rand::Rng;

use genome::{Gene, Genome};
use params::Params;
use species::Species;

/// The "classification" of a gene.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum GeneClass {
    /// The gene appears on both genes. The parameter is the absolute
    /// difference between the weight.
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
pub fn classify_genes(g1: &[Gene], g2: &[Gene]) -> Vec<GeneClass> {
    let inno_pos = |genes: &[Gene], inno: usize| {
        genes.iter().position(|g| g.innovation == inno)
    };
    let max_inno = |genes: &[Gene]| {
        genes
            .iter()
            .map(|g| g.innovation)
            .max()
            .unwrap_or(0)
    };

    let max_l = max_inno(g1);
    let max_r = max_inno(g2);
    let max_innovation = max(max_l, max_r);

    let mut gene_classes = Vec::new();
    for inno in 0..max_innovation {
        match (inno_pos(g1, inno), inno_pos(g2, inno)) {
            (Some(l), Some(r)) => {
                let weight_diff = (g1[l].weight - g2[r].weight).abs();
                gene_classes.push(GeneClass::Matching(weight_diff));
            }
            (Some(_), None) => {
                gene_classes.push(if inno > max_r {
                    GeneClass::ExcessLeft
                } else {
                    GeneClass::DisjointLeft
                });
            }
            (None, Some(_)) => {
                gene_classes.push(if inno > max_l {
                    GeneClass::ExcessRight
                } else {
                    GeneClass::DisjointRight
                });
            }
            (None, None) => {}
        }
    }
    gene_classes
}

/// Computes the difference between two organisms.
pub fn difference(g1: &Genome, g2: &Genome, params: &Params) -> f32 {
    let mut e = 0.0;
    let mut d = 0.0;
    let mut w_sum = 0.0;

    let classes = classify_genes(&g1.genes, &g2.genes);
    let n = if g1.len() < 20 && g2.len() < 20 {
        1.0
    } else {
        classes.len() as f32
    };

    for gene in classes {
        match gene {
            GeneClass::Matching(d) => w_sum += d,
            GeneClass::ExcessLeft | GeneClass::ExcessRight => e += 1.0,
            GeneClass::DisjointLeft | GeneClass::DisjointRight => d += 1.0,
        }
    }
    let w = w_sum / n;

    e /= n;
    d /= n;

    params.c1 * e + params.c2 * d + params.c3 * w
}

/// Classifies new organisms into species based on the previous species.
///
/// `organisms` and `fitnesses` must have the same length. Note that this will
/// panic if any empty species is provided. Don't do that.
pub fn classify_species<R: Rng>(
    rng: &mut R,
    organisms: Vec<Genome>,
    params: &Params,
    previous_species: &[Species],
) -> Vec<Species> {
    let mut species_count = previous_species.len();
    let species_assignments = {
        let mut exemplars = previous_species
            .iter()
            .map(|species| rng.choose(&species.0).unwrap())
            .collect::<Vec<_>>();

        organisms
            .iter()
            .map(|genome| {
                exemplars
                    .iter()
                    .position(|exemplar| {
                        difference(exemplar, genome, params)
                            < params.delta_cutoff
                    })
                    .unwrap_or_else(|| {
                        let n = species_count;
                        exemplars.push(genome);
                        species_count += 1;
                        n
                    })
            })
            .collect::<Vec<_>>()
    };

    let mut species = vec![Species::empty(); species_count];
    organisms
        .into_iter()
        .enumerate()
        .for_each(|(i, o)| {
            species[species_assignments[i]].0.push(o);
        });
    species.retain(|s| s.0.len() > 0);
    species
}
