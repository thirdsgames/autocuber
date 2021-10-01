use crate::{
    cube::{Move, MoveSequence, RotationType},
    group::{CyclicGroup, GroupAction, Unital},
    permute::{CubePermutation3, EdgeCubelet},
    solve::ActionSteps,
};

/// Uses Dijkstra's algorithm to search short move sequences.
/// Typically used for intuitively building blocks for Roux, Petrus and similar methods.
struct IntuitiveBlockbuilder {
    /// The full set of moves we are permitted to make on the cube.
    /// This is the generating set that we use to generate move sequences.
    ///
    /// These sequences may be more than a single move, e.g. R F R'.
    /// This allows us to perform certain predefined inserts without breaking
    /// existing blocks, for instance.
    gen_set: Vec<MoveSequence>,

    /// The graph that we use as the search space.
    graph: BlockbuildingGraph,
}

/// A graph used for blockbuilding is essentially a tree of move sequences.
struct BlockbuildingGraph {
    /// The permutation at the current cube state.
    permutation: CubePermutation3,
    /// Future move sequences.
    /// This is None if the children have not yet been explored.
    children: Option<Vec<(MoveSequence, BlockbuildingGraph)>>,
}

impl BlockbuildingGraph {
    fn new() -> Self {
        Self {
            permutation: CubePermutation3::identity(),
            children: None,
        }
    }

    /// Search for a given condition.
    /// Returns a list of move sequences with at most the given move count
    /// that all accomplish the given condition.
    fn search(
        &mut self,
        max_moves: usize,
        gen_set: &[MoveSequence],
        condition: impl Clone + Fn(&CubePermutation3) -> bool,
    ) -> Vec<(MoveSequence, CubePermutation3)> {
        let mut solutions = Vec::new();
        if condition(&self.permutation) {
            solutions.push((MoveSequence { moves: Vec::new() }, self.permutation));
        }

        let children = self.children.get_or_insert_with(|| {
            gen_set
                .iter()
                .cloned()
                .map(|moves| {
                    (
                        moves.clone(),
                        BlockbuildingGraph {
                            permutation: CubePermutation3::from_move_sequence(moves),
                            children: None,
                        },
                    )
                })
                .collect()
        });

        for (sequence, graph) in children {
            if sequence.moves.len() <= max_moves {
                let inner_solutions =
                    graph.search(max_moves - sequence.moves.len(), gen_set, condition.clone());
                solutions.extend(inner_solutions.into_iter().map(|(mut moves, permutation)| {
                    moves.moves.splice(0..0, sequence.moves.clone());
                    (moves, permutation)
                }))
            }
        }
        solutions
    }
}

impl IntuitiveBlockbuilder {
    /// Search the blockbuilding graph, adding more nodes if necessary,
    /// for a permutation that moves the given edge into the given slot.
    pub fn insert_edge(
        &mut self,
        source: EdgeCubelet,
        target: EdgeCubelet,
        target_orientation: CyclicGroup<2>,
    ) {
        let results = self.graph.search(3, &self.gen_set, |perm| {
            perm.edges().act(&(source, CyclicGroup::new(0))) == (target, target_orientation)
        });
        for (seq, _) in results {
            println!("{:#?}", seq);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::EdgeType::*;

    use super::*;

    #[test]
    fn test_edge_insert() {
        let mut blockbuilder = IntuitiveBlockbuilder {
            gen_set: vec![
                "F", "R", "U", "B", "L", "D", "F'", "R'", "U'", "B'", "L'", "D'", "F2", "R2", "U2",
                "B2", "L2", "D2",
            ]
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect(),
            graph: BlockbuildingGraph::new(),
        };
        blockbuilder.insert_edge(EdgeCubelet(UF), EdgeCubelet(FR), CyclicGroup::new(0));
        println!(
            "{}",
            CubePermutation3::from_move_sequence("U' R'".parse().unwrap())
        );
        panic!();
    }
}
