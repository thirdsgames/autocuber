use std::{collections::HashMap, hash::Hash};

use priority_queue::PriorityQueue;

use crate::{
    cube::{CornerType, EdgeType, MoveSequence},
    group::{CyclicGroup, GroupAction, InverseSemigroup, Magma, Unital},
    permute::{CornerCubelet, CubePermutation3, EdgeCubelet},
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

/// S is a 'signature' of the current cube state (see [SequenceGraph] for more info).
/// Query this object to get optimal move sequences for solving a cube into a specific (pre-determined) signature.
#[derive(Debug)]
pub struct SequenceSolver<S> {
    node_info: HashMap<S, MoveSequence>,
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
        gen_set: Vec<MoveSequence>,
        signature: impl Fn(CubePermutation3) -> S + Clone,
    ) -> Self {
        let mut this = Self::empty();

        // Generate double and inverse moves.
        let mut real_gen_set = gen_set
            .iter()
            .map(|mv| [mv.inverse(), mv.clone().op(mv.clone()), mv.clone()])
            .flatten()
            .map(|mv| mv.canonicalise())
            .filter(|mv| !mv.moves.is_empty())
            .collect::<Vec<_>>();
        real_gen_set.sort();
        real_gen_set.dedup();

        this.explore(&real_gen_set, CubePermutation3::identity(), signature);
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
    pub fn search(
        &self,
        target_signature: S,
        metric: impl Fn(&MoveSequence) -> u64,
    ) -> SequenceSolver<S> {
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

        while let Some((signature, _priority)) = unvisited_queue.pop() {
            // let distance = std::u64::MAX - priority;
            let move_sequence = unvisited_move_sequences
                .remove(signature)
                .expect("node was not given a move sequence but had max search priority");

            node_info.insert(signature.clone(), move_sequence.inverse());

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

        SequenceSolver { node_info }
    }
}

impl<S> SequenceSolver<S>
where
    S: Eq + Hash,
{
    /// Gives an optimal move sequence to solve the given signature into the target signature.
    pub fn solve(&self, signature: &S) -> Option<&MoveSequence> {
        self.node_info.get(signature)
    }
}

lazy_static::lazy_static! {
    pub static ref ROUX_FIRST_EDGE: SequenceSolver<(EdgeCubelet, CyclicGroup<2>)> = {
        let gen_set = vec!["F", "R", "U", "B", "L", "D", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        // Track all possible move sequences that influence a single edge (starting with the DL edge).
        let graph = SequenceGraph::new(gen_set, |cube| {
            cube.edges()
                .act(&(EdgeCubelet(EdgeType::DL), CyclicGroup::identity()))
        });
        graph.search((EdgeCubelet(EdgeType::DL), CyclicGroup::identity()), |seq| {
            seq.moves.len() as u64
        })
    };

    pub static ref ROUX_FIRST_PAIR: SequenceSolver<((EdgeCubelet, CyclicGroup<2>), (CornerCubelet, CyclicGroup<3>))> = {
        let gen_set = vec!["F", "R", "U", "B", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new(gen_set, |cube| {
            (
                cube.edges()
                    .act(&(EdgeCubelet(EdgeType::FL), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(CornerType::FDL), CyclicGroup::identity()))
            )
        });
        graph.search(((EdgeCubelet(EdgeType::FL), CyclicGroup::identity()), (CornerCubelet(CornerType::FDL), CyclicGroup::identity())), |seq| {
            seq.moves.len() as u64
        })
    };

    pub static ref ROUX_SECOND_PAIR: SequenceSolver<((EdgeCubelet, CyclicGroup<2>), (CornerCubelet, CyclicGroup<3>))> = {
        let gen_set = vec!["R", "U", "B", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new(gen_set, |cube| {
            (
                cube.edges()
                    .act(&(EdgeCubelet(EdgeType::BL), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(CornerType::BDL), CyclicGroup::identity()))
            )
        });
        graph.search(((EdgeCubelet(EdgeType::BL), CyclicGroup::identity()), (CornerCubelet(CornerType::BDL), CyclicGroup::identity())), |seq| {
            seq.moves.len() as u64
        })
    };

    pub static ref ROUX_SECOND_EDGE: SequenceSolver<(EdgeCubelet, CyclicGroup<2>)> = {
        let gen_set = vec!["R", "U", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        // Track all possible move sequences that influence a single edge (starting with the DL edge).
        let graph = SequenceGraph::new(gen_set, |cube| {
            cube.edges()
                .act(&(EdgeCubelet(EdgeType::DR), CyclicGroup::identity()))
        });
        graph.search((EdgeCubelet(EdgeType::DR), CyclicGroup::identity()), |seq| {
            seq.moves.len() as u64
        })
    };

    pub static ref ROUX_THIRD_PAIR: SequenceSolver<((EdgeCubelet, CyclicGroup<2>), (CornerCubelet, CyclicGroup<3>))> = {
        let gen_set = vec!["U", "M", "R U R'", "R U2 R'", "R U' R'"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new(gen_set, |cube| {
            (
                cube.edges()
                    .act(&(EdgeCubelet(EdgeType::FR), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(CornerType::FDR), CyclicGroup::identity()))
            )
        });
        graph.search(((EdgeCubelet(EdgeType::FR), CyclicGroup::identity()), (CornerCubelet(CornerType::FDR), CyclicGroup::identity())), |seq| {
            seq.moves.len() as u64
        })
    };

    pub static ref ROUX_FOURTH_PAIR: SequenceSolver<((EdgeCubelet, CyclicGroup<2>), (CornerCubelet, CyclicGroup<3>))> = {
        let gen_set = vec!["U", "M", "R' U R", "R' U2 R", "R' U' R"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new(gen_set, |cube| {
            (
                cube.edges()
                    .act(&(EdgeCubelet(EdgeType::BR), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(CornerType::BDR), CyclicGroup::identity()))
            )
        });
        graph.search(((EdgeCubelet(EdgeType::BR), CyclicGroup::identity()), (CornerCubelet(CornerType::BDR), CyclicGroup::identity())), |seq| {
            seq.moves.len() as u64
        })
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        cube::{CornerType::*, EdgeType::*},
        group::{CyclicGroup, GroupAction},
        permute::EdgeCubelet,
    };

    use super::*;

    #[test]
    fn test_edge_insert() {
        // Solve the DF edge piece (oriented badly) into the DL slot.
        let solution = ROUX_FIRST_EDGE.solve(&(EdgeCubelet(DF), CyclicGroup::new(1)));
        assert_eq!(
            CubePermutation3::from_move_sequence(solution.unwrap().clone())
                .edges()
                .act(&(EdgeCubelet(DF), CyclicGroup::new(1))),
            (EdgeCubelet(DL), CyclicGroup::new(0))
        );

        // Solve the UB edge piece (oriented correctly) into the DL slot.
        let solution = ROUX_FIRST_EDGE.solve(&(EdgeCubelet(UB), CyclicGroup::new(0)));
        assert_eq!(
            CubePermutation3::from_move_sequence(solution.unwrap().clone())
                .edges()
                .act(&(EdgeCubelet(UB), CyclicGroup::new(0))),
            (EdgeCubelet(DL), CyclicGroup::new(0))
        );
    }

    #[test]
    fn roux_first_pair() {
        // Scramble the cube.
        let scramble: MoveSequence =
            "B R2 U2 F R' U' B2 F U R2 U2 L' D' R2 D L R' F' R F2 B2 U D' R L2"
                .parse()
                .unwrap();

        let mut permutation = CubePermutation3::from_move_sequence(scramble);

        // Solve the first edge.
        let solution_first_edge = ROUX_FIRST_EDGE
            .solve(
                &permutation
                    .edges()
                    .act(&(EdgeCubelet(DL), CyclicGroup::identity())),
            )
            .unwrap();
        println!("First edge: {}", solution_first_edge);
        permutation =
            CubePermutation3::from_move_sequence(solution_first_edge.clone()).op(permutation);

        // Solve the first pair.
        let solution_first_pair = ROUX_FIRST_PAIR
            .solve(&(
                permutation
                    .edges()
                    .act(&(EdgeCubelet(FL), CyclicGroup::identity())),
                permutation
                    .corners()
                    .act(&(CornerCubelet(FDL), CyclicGroup::identity())),
            ))
            .unwrap();
        println!("First pair: {}", solution_first_pair);
        permutation =
            CubePermutation3::from_move_sequence(solution_first_pair.clone()).op(permutation);

        // Solve the second pair.
        let solution_second_pair = ROUX_SECOND_PAIR
            .solve(&(
                permutation
                    .edges()
                    .act(&(EdgeCubelet(BL), CyclicGroup::identity())),
                permutation
                    .corners()
                    .act(&(CornerCubelet(BDL), CyclicGroup::identity())),
            ))
            .unwrap();
        println!("Second pair: {}", solution_second_pair);
        permutation =
            CubePermutation3::from_move_sequence(solution_second_pair.clone()).op(permutation);

        // Solve the second edge.
        let solution_second_edge = ROUX_SECOND_EDGE
            .solve(
                &permutation
                    .edges()
                    .act(&(EdgeCubelet(DR), CyclicGroup::identity())),
            )
            .unwrap();
        println!("Second edge: {}", solution_second_edge);
        permutation =
            CubePermutation3::from_move_sequence(solution_second_edge.clone()).op(permutation);

        // Solve the third pair.
        let solution_third_pair = ROUX_THIRD_PAIR
            .solve(&(
                permutation
                    .edges()
                    .act(&(EdgeCubelet(FR), CyclicGroup::identity())),
                permutation
                    .corners()
                    .act(&(CornerCubelet(FDR), CyclicGroup::identity())),
            ))
            .unwrap();
        println!("Third pair: {}", solution_third_pair);
        permutation =
            CubePermutation3::from_move_sequence(solution_third_pair.clone()).op(permutation);

        // Solve the fourth pair.
        let solution_fourth_pair = ROUX_FOURTH_PAIR
            .solve(&(
                permutation
                    .edges()
                    .act(&(EdgeCubelet(BR), CyclicGroup::identity())),
                permutation
                    .corners()
                    .act(&(CornerCubelet(BDR), CyclicGroup::identity())),
            ))
            .unwrap();
        println!("Fourth pair: {}", solution_fourth_pair);
        permutation =
            CubePermutation3::from_move_sequence(solution_fourth_pair.clone()).op(permutation);

        for edge in [FL, FR, BL, BR, DL, DR] {
            assert_eq!(
                permutation
                    .edges()
                    .act(&(EdgeCubelet(edge), CyclicGroup::identity())),
                (EdgeCubelet(edge), CyclicGroup::identity())
            );
        }

        for corner in [FDL, FDR, BDL, BDR] {
            assert_eq!(
                permutation
                    .corners()
                    .act(&(CornerCubelet(corner), CyclicGroup::identity())),
                (CornerCubelet(corner), CyclicGroup::identity())
            );
        }
    }
}
