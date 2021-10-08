use crate::{
    cube::{CornerType, EdgeType, FaceType, MoveSequence},
    group::{CyclicGroup, GroupAction, Unital},
    intuitive::{SequenceGraph, SequenceSolver},
    permute::{CentreCubelet, CornerCubelet, EdgeCubelet},
};

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
        let gen_set = vec!["U", "M", "R U R'", "R U2 R'", "R U' R'", "R' U R", "R' U2 R", "R' U' R"]
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

    /// Solves CMLL and then adjusts the U face.
    static ref CMLL_AUF: SequenceSolver<[(CornerCubelet, CyclicGroup<3>); 4]> = {
        let gen_set = vec![
            // AUF
            "U",
            // J perm
            "R U R' F' R U R' U' R' F R2 U' R'",
            // Y perm
            "F R U' R' U' R U R' F' R U R' U' R' F R F'",
            // Antisune
            "R' U' R U' R' U2 R",
            // Sune
            "R U R' U R U2 R'",
            // L
            "R' F2 R' U' R F2 R' U R2",
            // Sexy move cases
            "F R U R' U' F'",
            "F R U R' U' R U R' U' F'",
            "F R U R' U' R U R' U' R U R' U' F'",
            ]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new(gen_set, |cube| {
            [
                cube.corners()
                    .act(&(CornerCubelet(CornerType::FUL), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(CornerType::FUR), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(CornerType::BUR), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(CornerType::BUL), CyclicGroup::identity())),
            ]
        });
        graph.search([
            (CornerCubelet(CornerType::FUL), CyclicGroup::identity()),
            (CornerCubelet(CornerType::FUR), CyclicGroup::identity()),
            (CornerCubelet(CornerType::BUR), CyclicGroup::identity()),
            (CornerCubelet(CornerType::BUL), CyclicGroup::identity()),
        ], |seq| {
            seq.moves.len() as u64
        })
    };

    /// The signature is
    /// - EO of UF UB DB DF,
    /// - the edges UL UR,
    /// - the FUL corner position (used for AUF),
    /// - and whether the front face is the F/B colour (true) or not (false).
    pub static ref EOLR: SequenceSolver<([CyclicGroup<2>; 4], [(EdgeCubelet, CyclicGroup<2>); 2], CornerCubelet, bool)> = {
        let gen_set = vec!["U", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new(gen_set, |cube| {
            (
                [
                    // Unact is used to get edge orientation: we don't care which edge is in this position,
                    // just how it is oriented relative to where the edge should belong.
                    cube.edges()
                       .unact(&(EdgeCubelet(EdgeType::UF), CyclicGroup::identity())).1,
                    cube.edges()
                       .unact(&(EdgeCubelet(EdgeType::UB), CyclicGroup::identity())).1,
                    cube.edges()
                       .unact(&(EdgeCubelet(EdgeType::DB), CyclicGroup::identity())).1,
                    cube.edges()
                       .unact(&(EdgeCubelet(EdgeType::DF), CyclicGroup::identity())).1,
                ],
                [
                    cube.edges()
                        .act(&(EdgeCubelet(EdgeType::UL), CyclicGroup::identity())),
                    cube.edges()
                        .act(&(EdgeCubelet(EdgeType::UR), CyclicGroup::identity())),
                ],
                cube.corners()
                    .act(&(CornerCubelet(CornerType::FUL), CyclicGroup::identity())).0,
                matches!(cube.centres().act(&CentreCubelet(FaceType::F)).0, FaceType::F | FaceType::B),
            )
        });
        graph.search((
            [CyclicGroup::identity(); 4],
            [(EdgeCubelet(EdgeType::UL), CyclicGroup::identity()), (EdgeCubelet(EdgeType::UR), CyclicGroup::identity())],
            CornerCubelet(CornerType::FUL),
            true,
        ), |seq| {
            seq.moves.len() as u64
        })
    };

    /// The signature is the last four edges' positions (UF UB DB DF), and the front-facing centre.
    pub static ref L4E: SequenceSolver<([EdgeCubelet; 4], CentreCubelet)> = {
        let gen_set = vec![
            "U2 M U2 M",
            "U2 M' U2 M",
            "U2 M U2 M'",
            "U2 M' U2 M'",
            "U2 M2 U2",
            "M' U2 M2 U2 M",
            "M' U2 M2 U2 M'",
            "E2 M E2 M",
            "E2 M E2 M'",
            "M2"
        ]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new(gen_set, |cube| {
            (
                [
                    cube.edges().act(&(EdgeCubelet(EdgeType::UF), CyclicGroup::identity())).0,
                    cube.edges().act(&(EdgeCubelet(EdgeType::UB), CyclicGroup::identity())).0,
                    cube.edges().act(&(EdgeCubelet(EdgeType::DB), CyclicGroup::identity())).0,
                    cube.edges().act(&(EdgeCubelet(EdgeType::DF), CyclicGroup::identity())).0,
                ],
                cube.centres().act(&CentreCubelet(FaceType::F)),
            )
        });
        graph.search((
            [
                EdgeCubelet(EdgeType::UF),
                EdgeCubelet(EdgeType::UB),
                EdgeCubelet(EdgeType::DB),
                EdgeCubelet(EdgeType::DF),
            ],
            CentreCubelet(FaceType::F),
        ), |seq| {
            seq.moves.len() as u64
        })
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        cube::{CornerType::*, EdgeType::*},
        group::{CyclicGroup, GroupAction, Magma},
        permute::{CubePermutation3, EdgeCubelet},
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
    fn roux_two_blocks() {
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

        // Solve CMLL.
        let solution_cmll = CMLL_AUF
            .solve(&[
                permutation
                    .corners()
                    .act(&(CornerCubelet(CornerType::FUL), CyclicGroup::identity())),
                permutation
                    .corners()
                    .act(&(CornerCubelet(CornerType::FUR), CyclicGroup::identity())),
                permutation
                    .corners()
                    .act(&(CornerCubelet(CornerType::BUR), CyclicGroup::identity())),
                permutation
                    .corners()
                    .act(&(CornerCubelet(CornerType::BUL), CyclicGroup::identity())),
            ])
            .unwrap();
        println!("CMLL: {}", solution_cmll);
        permutation = CubePermutation3::from_move_sequence(solution_cmll.clone()).op(permutation);

        // Solve EOLR.
        let solution_eolr = EOLR
            .solve(&(
                [
                    permutation
                        .edges()
                        .unact(&(EdgeCubelet(EdgeType::UF), CyclicGroup::identity()))
                        .1,
                    permutation
                        .edges()
                        .unact(&(EdgeCubelet(EdgeType::UB), CyclicGroup::identity()))
                        .1,
                    permutation
                        .edges()
                        .unact(&(EdgeCubelet(EdgeType::DB), CyclicGroup::identity()))
                        .1,
                    permutation
                        .edges()
                        .unact(&(EdgeCubelet(EdgeType::DF), CyclicGroup::identity()))
                        .1,
                ],
                [
                    permutation
                        .edges()
                        .act(&(EdgeCubelet(EdgeType::UL), CyclicGroup::identity())),
                    permutation
                        .edges()
                        .act(&(EdgeCubelet(EdgeType::UR), CyclicGroup::identity())),
                ],
                permutation
                    .corners()
                    .act(&(CornerCubelet(CornerType::FUL), CyclicGroup::identity()))
                    .0,
                matches!(
                    permutation.centres().act(&CentreCubelet(FaceType::F)).0,
                    FaceType::F | FaceType::B
                ),
            ))
            .unwrap();
        println!("EOLR: {}", solution_eolr);
        permutation = CubePermutation3::from_move_sequence(solution_eolr.clone()).op(permutation);

        // Solve L4E.
        let solution_l4e = L4E
            .solve(&(
                [
                    permutation
                        .edges()
                        .act(&(EdgeCubelet(EdgeType::UF), CyclicGroup::identity()))
                        .0,
                    permutation
                        .edges()
                        .act(&(EdgeCubelet(EdgeType::UB), CyclicGroup::identity()))
                        .0,
                    permutation
                        .edges()
                        .act(&(EdgeCubelet(EdgeType::DB), CyclicGroup::identity()))
                        .0,
                    permutation
                        .edges()
                        .act(&(EdgeCubelet(EdgeType::DF), CyclicGroup::identity()))
                        .0,
                ],
                permutation.centres().act(&CentreCubelet(FaceType::F)),
            ))
            .unwrap();
        println!("L4E: {}", solution_l4e);
        permutation = CubePermutation3::from_move_sequence(solution_l4e.clone()).op(permutation);

        assert_eq!(permutation, CubePermutation3::identity());
    }
}
