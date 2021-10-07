use std::{collections::HashMap, hash::Hash};

use priority_queue::PriorityQueue;

use crate::{
    cube::MoveSequence,
    group::{InverseSemigroup, Magma, Unital},
    permute::CubePermutation3,
};

/// S is a 'signature' of the current cube state.
/// It should be a data type that specifies some limited information about the cube state.
#[derive(Debug)]
pub struct SequenceGraph<S> {
    graph: HashMap<S, State<S>>,
}

#[derive(Debug)]
struct State<S> {
    /// Given a sequence of moves, which node do we transition to?
    /// Move sequences that transition to the same node are omitted.
    transitions: HashMap<MoveSequence, S>,
}

impl<S> SequenceGraph<S> {
    /// Create a new *empty* sequence graph.
    fn empty() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }
}

impl<S> SequenceGraph<S>
where
    S: Eq + Hash + Clone,
{
    /// Create a new sequence graph from the given generating set.
    /// For each generated move sequence, we generate the signature of the resulting cube permutation.
    /// The signature function should generate the signature of a cube permutation.
    pub fn new(
        gen_set: &[MoveSequence],
        signature: impl Fn(CubePermutation3) -> S + Clone,
    ) -> Self {
        let mut this = Self::empty();
        this.explore(gen_set, CubePermutation3::identity(), signature);
        this
    }

    fn explore(
        &mut self,
        gen_set: &[MoveSequence],
        permutation: CubePermutation3,
        signature: impl Fn(CubePermutation3) -> S + Clone,
    ) {
        let current_signature = signature(permutation);
        let mut new_permutations = Vec::new();
        self.graph
            .entry(current_signature.clone())
            .or_insert_with(|| {
                let mut state = State {
                    transitions: HashMap::default(),
                };
                // Try each move in the generating set.
                // Check what the signature is.
                for seq in gen_set {
                    let seq_perm = CubePermutation3::from_move_sequence(seq.clone());
                    let new_permutation = seq_perm.op(permutation);
                    let new_signature = signature(new_permutation);

                    if new_signature != current_signature {
                        new_permutations.push(new_permutation);
                        state.transitions.insert(seq.clone(), new_signature);
                    }
                }
                state
            });

        for new_permutation in new_permutations {
            self.explore(gen_set, new_permutation, signature.clone());
        }
    }

    /// Searches the sequence graph using Dijkstra's algorithm
    /// to provide (essentially) a lookup table containing the shortest move sequences that will
    /// repair the cube to a specific 'target' signature.
    ///
    /// The metric gives a score to each move sequence.
    /// Lower is better.
    /// The target signature should have a metric of zero.
    /// A typical example of a metric is STM, or 'slice turn metric'.
    pub fn search(&self, target_signature: S, metric: impl Fn(&MoveSequence) -> u64) {
        // The set of unvisited nodes, ordered by current distance.
        // The priority of an element is given by `std::u64::MAX` minus the distance.
        let mut unvisited_queue = self
            .graph
            .keys()
            .map(|s| (s, 0))
            .collect::<PriorityQueue<_, _>>();

        // Stores the tentative move sequences used to reach each unvisited node
        // with the distance stored in the unvisited queue.
        let mut unvisited_move_sequences = HashMap::new();

        // Add in the unvisited queue entry for the target signature.
        // It should have distance zero, so max priority.
        unvisited_queue.change_priority(&target_signature, std::u64::MAX);
        unvisited_move_sequences.insert(&target_signature, MoveSequence { moves: Vec::new() });

        // The distance and move sequence for each visited signature node.
        // Node info and the unvisited queue are mutually exclusive.
        // Their union is the set of all S.
        // Note that the given move sequence is the reverse of the move sequence in unvisited_move_sequences:
        // this move sequence will repair the cube into the target signature, wheread unvisited_move_sequences
        // will convert the cube from the target signature into the given signature.
        let mut node_info = HashMap::new();

        while let Some((signature, priority)) = unvisited_queue.pop() {
            let distance = std::u64::MAX - priority;
            let move_sequence = unvisited_move_sequences
                .remove(signature)
                .expect("node was not given a move sequence but had max search priority");
            // println!(
            //     "Found {:#?} with distance {}, sequence {:#?}",
            //     signature, priority, move_sequence
            // );

            node_info.insert(signature.clone(), (distance, move_sequence.inverse()));

            // For the current node, consider all of its unvisited neighbours.
            for (transition_sequence, new_signature) in &self.graph[signature].transitions {
                if let Some(&existing_priority) = unvisited_queue.get_priority(new_signature) {
                    // This is an unvisited node.

                    let tentative_move_sequence = MoveSequence {
                        moves: move_sequence
                            .moves
                            .iter()
                            .chain(&transition_sequence.moves)
                            .cloned()
                            .collect(),
                    };
                    let tentative_metric = metric(&tentative_move_sequence);
                    let tentative_priority = std::u64::MAX - tentative_metric;
                    if tentative_priority > existing_priority {
                        // We found a better route to this signature.
                        unvisited_queue.change_priority(new_signature, tentative_priority);
                        unvisited_move_sequences.insert(new_signature, tentative_move_sequence);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cube::EdgeType::*,
        group::{CyclicGroup, GroupAction},
        permute::EdgeCubelet,
    };

    use super::*;

    #[test]
    fn test_edge_insert() {
        let gen_set = vec![
            "F", "R", "U", "B", "L", "D", "F'", "R'", "U'", "B'", "L'", "D'", "F2", "R2", "U2",
            "B2", "L2", "D2",
        ]
        .into_iter()
        .map(|x| x.parse::<MoveSequence>().unwrap())
        .collect::<Vec<_>>();

        // Track all possible move sequences that influence a single edge (starting with the DL edge).
        let graph = SequenceGraph::new(&gen_set, |cube| {
            cube.edges()
                .act(&(EdgeCubelet(DL), CyclicGroup::identity()))
        });
        let search_result = graph.search((EdgeCubelet(DL), CyclicGroup::identity()), |seq| {
            seq.moves.len() as u64
        });
        println!("{:#?}", search_result);

        panic!();
    }
}
