use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    time::Instant,
};

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
    graph_name: &'static str,
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

impl<S> SequenceGraph<S>
where
    S: Eq + Hash + Clone,
{
    /// Create a new sequence graph from the given generating set.
    /// For each generated move sequence, we generate the signature of the resulting cube permutation.
    /// The signature function should generate the signature of a cube permutation.
    pub fn new(
        graph_name: &'static str,
        gen_set: Vec<MoveSequence>,
        signature: impl Fn(CubePermutation3) -> S,
    ) -> Self {
        let start_time = Instant::now();

        let mut this = Self {
            graph: HashMap::new(),
            graph_name,
        };

        // Generate double and inverse moves.
        let mut real_gen_set = gen_set
            .iter()
            .map(|mv| {
                if mv.moves.len() > 1 {
                    // Don't generate inverses etc. for full algorithms or conjugates.
                    // These algorithms must, however, be reversed.
                    // This is because the move sequences themselves will be reversed when solving as opposed to exploring.
                    vec![mv.inverse()]
                } else {
                    vec![mv.inverse(), mv.clone().op(mv.clone()), mv.clone()]
                }
            })
            .flatten()
            .map(|mv| mv.canonicalise())
            .filter(|mv| !mv.moves.is_empty())
            .collect::<Vec<_>>();
        real_gen_set.sort();
        real_gen_set.dedup();

        let mut new_permutations = VecDeque::new();
        // Initialise the list of permutations with the identity,
        // so we have a source to explore from.
        new_permutations.push_back(CubePermutation3::identity());

        while let Some(permutation) = new_permutations.pop_front() {
            let current_signature = signature(permutation);
            this.graph
                .entry(current_signature.clone())
                .or_insert_with(|| {
                    let mut state = State {
                        transitions: HashMap::default(),
                    };
                    // Try each move in the generating set.
                    // Check what the signature is.
                    for seq in &real_gen_set {
                        let seq_perm = CubePermutation3::from_move_sequence(seq.clone());
                        let new_permutation = seq_perm.op(permutation);
                        let new_signature = signature(new_permutation);

                        if new_signature != current_signature {
                            new_permutations.push_back(new_permutation);
                            state.transitions.insert(seq.clone(), new_signature);
                        }
                    }
                    state
                });
        }

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!(
            "Generated sequence graph {} with {} nodes in {} ms",
            graph_name,
            this.graph.len(),
            duration.as_millis()
        );

        this
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
        let start_time = Instant::now();

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

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!(
            "Searched sequence graph {} in {} ms",
            self.graph_name,
            duration.as_millis()
        );

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
