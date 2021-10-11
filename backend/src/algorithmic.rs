use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

use instant::Instant;

use crate::{
    cube::MoveSequence,
    group::{InverseSemigroup, Magma},
    permute::CubePermutation3,
    utils::log,
};

/// S is a 'signature' of the current cube state (see [crate::intuitive::SequenceGraph] for more info).
/// Query this object to get optimal move sequences for solving a cube into a specific (pre-determined) signature.
#[derive(Debug)]
pub struct AlgorithmicSolver<S> {
    node_info: HashMap<S, MoveSequence>,
}

impl<S> AlgorithmicSolver<S>
where
    S: Eq + Hash,
{
    /// Create a new sequence graph from the given generating set.
    /// For each generated move sequence, we generate the signature of the resulting cube permutation.
    /// The signature function should generate the signature of a cube permutation.
    /// The set of post moves (typically AUF moves) are used for matching signatures, but are elided in the move sequences generated.
    pub fn new(
        graph_name: &'static str,
        alg_set: Vec<MoveSequence>,
        pre_moves: Vec<MoveSequence>,
        post_moves: Vec<MoveSequence>,
        signature: impl Fn(CubePermutation3) -> S,
        metric: impl Fn(&MoveSequence) -> u64,
    ) -> Self {
        let start_time = Instant::now();

        let mut this = Self {
            node_info: HashMap::new(),
        };

        let mut real_pre_moves = pre_moves
            .iter()
            .map(|mv| vec![mv.inverse(), mv.clone().op(mv.clone()), mv.clone()])
            .flatten()
            .map(|mv| mv.canonicalise())
            .chain(std::iter::once(MoveSequence { moves: Vec::new() }))
            .collect::<Vec<_>>();
        real_pre_moves.sort();
        real_pre_moves.dedup();

        let mut real_post_moves = post_moves
            .iter()
            .map(|mv| vec![mv.inverse(), mv.clone().op(mv.clone()), mv.clone()])
            .flatten()
            .map(|mv| mv.canonicalise())
            .chain(std::iter::once(MoveSequence { moves: Vec::new() }))
            .collect::<Vec<_>>();
        real_post_moves.sort();
        real_post_moves.dedup();

        for alg in alg_set {
            for pre_move in &real_pre_moves {
                for post_move in &real_post_moves {
                    let moves_no_pre = post_move.clone().op(alg.clone());
                    let moves_no_pre_inverse = moves_no_pre.inverse();
                    let moves = moves_no_pre.op(pre_move.clone());
                    let sig = signature(CubePermutation3::from_move_sequence(moves));
                    match this.node_info.entry(sig) {
                        Entry::Occupied(mut entry) => {
                            // If two move sequences gave the same result, shorter is better.
                            let new_metric = metric(&moves_no_pre_inverse);
                            let previous_metric = metric(entry.get());
                            if new_metric < previous_metric {
                                // Replace with the new entry.
                                entry.insert(moves_no_pre_inverse);
                            }
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(moves_no_pre_inverse);
                        }
                    }
                }
            }
        }

        let end_time = Instant::now();
        let duration = end_time - start_time;
        log!(
            "Generated algorithmic solver {} with {} nodes in {} ms",
            graph_name,
            this.node_info.len(),
            duration.as_millis()
        );

        this
    }

    pub fn solve(&self, signature: &S) -> Option<&MoveSequence> {
        self.node_info.get(signature)
    }
}
