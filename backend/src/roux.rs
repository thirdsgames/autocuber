use crate::{
    cube::{
        Axis, FaceType, Move, MoveSequence,
        {CornerType::*, EdgeType::*},
    },
    group::{CyclicGroup, GroupAction, Unital},
    intuitive::{SequenceGraph, SequenceSolver},
    permute::{CentreCubelet, CornerCubelet, CubePermutation3, EdgeCubelet},
};

type RouxEdgeSignature = (EdgeCubelet, CyclicGroup<2>);
type RouxPairSignature = (
    (EdgeCubelet, CyclicGroup<2>),
    (CornerCubelet, CyclicGroup<3>),
);
type RouxCmllSignature = [(CornerCubelet, CyclicGroup<3>); 4];
type RouxEolrSignature = ([CyclicGroup<2>; 6], [EdgeCubelet; 2], CornerCubelet, bool);
type RouxL4eSignature = ([EdgeCubelet; 4], CentreCubelet);

lazy_static::lazy_static! {
    static ref ROUX_FIRST_EDGE: SequenceSolver<RouxEdgeSignature> = {
        let gen_set = vec!["F", "R", "U", "B", "L", "D", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        // Track all possible move sequences that influence a single edge (starting with the DL edge).
        let graph = SequenceGraph::new("roux1e", gen_set, |cube| {
            cube.edges()
                .act(&(EdgeCubelet(DL), CyclicGroup::identity()))
        });
        graph.search((EdgeCubelet(DL), CyclicGroup::identity()), |seq| {
            seq.moves.len() as u64
        })
    };

    static ref ROUX_FIRST_PAIR: SequenceSolver<RouxPairSignature> = {
        let gen_set = vec!["F", "R", "U", "B", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new("roux1p", gen_set, |cube| {
            (
                cube.edges()
                    .act(&(EdgeCubelet(FL), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(FDL), CyclicGroup::identity()))
            )
        });
        graph.search(((EdgeCubelet(FL), CyclicGroup::identity()), (CornerCubelet(FDL), CyclicGroup::identity())), |seq| {
            seq.moves.len() as u64
        })
    };

    static ref ROUX_SECOND_PAIR: SequenceSolver<RouxPairSignature> = {
        let gen_set = vec!["R", "U", "B", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new("roux2p", gen_set, |cube| {
            (
                cube.edges()
                    .act(&(EdgeCubelet(BL), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(BDL), CyclicGroup::identity()))
            )
        });
        graph.search(((EdgeCubelet(BL), CyclicGroup::identity()), (CornerCubelet(BDL), CyclicGroup::identity())), |seq| {
            seq.moves.len() as u64
        })
    };

    static ref ROUX_SECOND_EDGE: SequenceSolver<RouxEdgeSignature> = {
        let gen_set = vec!["R", "U", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        // Track all possible move sequences that influence a single edge (starting with the DL edge).
        let graph = SequenceGraph::new("roux2e", gen_set, |cube| {
            cube.edges()
                .act(&(EdgeCubelet(DR), CyclicGroup::identity()))
        });
        graph.search((EdgeCubelet(DR), CyclicGroup::identity()), |seq| {
            seq.moves.len() as u64
        })
    };

    static ref ROUX_THIRD_PAIR: SequenceSolver<RouxPairSignature> = {
        let gen_set = vec!["U", "M", "R U R'", "R U2 R'", "R U' R'", "R' U R", "R' U2 R", "R' U' R"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new("roux3p", gen_set, |cube| {
            (
                cube.edges()
                    .act(&(EdgeCubelet(FR), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(FDR), CyclicGroup::identity()))
            )
        });
        graph.search(((EdgeCubelet(FR), CyclicGroup::identity()), (CornerCubelet(FDR), CyclicGroup::identity())), |seq| {
            seq.moves.len() as u64
        })
    };

    static ref ROUX_FOURTH_PAIR: SequenceSolver<RouxPairSignature> = {
        let gen_set = vec!["U", "M", "R' U R", "R' U2 R", "R' U' R"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new("roux4p", gen_set, |cube| {
            (
                cube.edges()
                    .act(&(EdgeCubelet(BR), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(BDR), CyclicGroup::identity()))
            )
        });
        graph.search(((EdgeCubelet(BR), CyclicGroup::identity()), (CornerCubelet(BDR), CyclicGroup::identity())), |seq| {
            seq.moves.len() as u64
        })
    };

    /// Solves CMLL and then adjusts the U face.
    static ref CMLL_AUF: SequenceSolver<RouxCmllSignature> = {
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

        let graph = SequenceGraph::new("roux_cmll", gen_set, |cube| {
            [
                cube.corners()
                    .act(&(CornerCubelet(FUL), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(FUR), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(BUR), CyclicGroup::identity())),
                cube.corners()
                    .act(&(CornerCubelet(BUL), CyclicGroup::identity())),
            ]
        });
        graph.search([
            (CornerCubelet(FUL), CyclicGroup::identity()),
            (CornerCubelet(FUR), CyclicGroup::identity()),
            (CornerCubelet(BUR), CyclicGroup::identity()),
            (CornerCubelet(BUL), CyclicGroup::identity()),
        ], |seq| {
            seq.moves.len() as u64
        })
    };

    /// The signature is
    /// - EO of positions UF UB DB DF UL UR,
    /// - the position of edges UL UR,
    /// - the FUL corner position (used for AUF),
    /// - and whether the front face is the F/B colour (true) or not (false).
    static ref EOLR: SequenceSolver<RouxEolrSignature> = {
        let gen_set = vec!["U", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new("roux_eolr", gen_set, |cube| {
            (
                [
                    // Unact is used to get edge orientation: we don't care which edge is in this position,
                    // just how it is oriented relative to where the edge should belong.
                    cube.edges()
                       .unact(&(EdgeCubelet(UF), CyclicGroup::identity())).1,
                    cube.edges()
                       .unact(&(EdgeCubelet(UB), CyclicGroup::identity())).1,
                    cube.edges()
                       .unact(&(EdgeCubelet(DB), CyclicGroup::identity())).1,
                    cube.edges()
                       .unact(&(EdgeCubelet(DF), CyclicGroup::identity())).1,
                    cube.edges()
                       .unact(&(EdgeCubelet(UL), CyclicGroup::identity())).1,
                    cube.edges()
                       .unact(&(EdgeCubelet(UR), CyclicGroup::identity())).1,
                ],
                [
                    cube.edges()
                        .act(&(EdgeCubelet(UL), CyclicGroup::identity())).0,
                    cube.edges()
                        .act(&(EdgeCubelet(UR), CyclicGroup::identity())).0,
                ],
                cube.corners()
                    .act(&(CornerCubelet(FUL), CyclicGroup::identity())).0,
                matches!(cube.centres().act(&CentreCubelet(FaceType::F)).0, FaceType::F | FaceType::B),
            )
        });
        graph.search((
            [CyclicGroup::identity(); 6],
            [EdgeCubelet(UL), EdgeCubelet(UR)],
            CornerCubelet(FUL),
            true,
        ), |seq| {
            seq.moves.len() as u64
        })
    };

    /// The signature is the last four edges' positions (UF UB DB DF), and the front-facing centre.
    static ref L4E: SequenceSolver<RouxL4eSignature> = {
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

        let graph = SequenceGraph::new("roux_l4e", gen_set, |cube| {
            (
                [
                    cube.edges().act(&(EdgeCubelet(UF), CyclicGroup::identity())).0,
                    cube.edges().act(&(EdgeCubelet(UB), CyclicGroup::identity())).0,
                    cube.edges().act(&(EdgeCubelet(DB), CyclicGroup::identity())).0,
                    cube.edges().act(&(EdgeCubelet(DF), CyclicGroup::identity())).0,
                ],
                cube.centres().act(&CentreCubelet(FaceType::F)),
            )
        });
        graph.search((
            [
                EdgeCubelet(UF),
                EdgeCubelet(UB),
                EdgeCubelet(DB),
                EdgeCubelet(DF),
            ],
            CentreCubelet(FaceType::F),
        ), |seq| {
            seq.moves.len() as u64
        })
    };
}

pub fn first_edge(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    ROUX_FIRST_EDGE.solve(
        &permutation
            .edges()
            .act(&(EdgeCubelet(DL), CyclicGroup::identity())),
    )
}

pub fn first_pair(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    ROUX_FIRST_PAIR.solve(&(
        permutation
            .edges()
            .act(&(EdgeCubelet(FL), CyclicGroup::identity())),
        permutation
            .corners()
            .act(&(CornerCubelet(FDL), CyclicGroup::identity())),
    ))
}

pub fn second_pair(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    ROUX_SECOND_PAIR.solve(&(
        permutation
            .edges()
            .act(&(EdgeCubelet(BL), CyclicGroup::identity())),
        permutation
            .corners()
            .act(&(CornerCubelet(BDL), CyclicGroup::identity())),
    ))
}

pub fn second_edge(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    ROUX_SECOND_EDGE.solve(
        &permutation
            .edges()
            .act(&(EdgeCubelet(DR), CyclicGroup::identity())),
    )
}

pub fn third_pair(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    ROUX_THIRD_PAIR.solve(&(
        permutation
            .edges()
            .act(&(EdgeCubelet(FR), CyclicGroup::identity())),
        permutation
            .corners()
            .act(&(CornerCubelet(FDR), CyclicGroup::identity())),
    ))
}

pub fn fourth_pair(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    ROUX_FOURTH_PAIR.solve(&(
        permutation
            .edges()
            .act(&(EdgeCubelet(BR), CyclicGroup::identity())),
        permutation
            .corners()
            .act(&(CornerCubelet(BDR), CyclicGroup::identity())),
    ))
}

pub fn cmll(permutation: CubePermutation3) -> Option<MoveSequence> {
    let cmll = CMLL_AUF.solve(&[
        permutation
            .corners()
            .act(&(CornerCubelet(FUL), CyclicGroup::identity())),
        permutation
            .corners()
            .act(&(CornerCubelet(FUR), CyclicGroup::identity())),
        permutation
            .corners()
            .act(&(CornerCubelet(BUR), CyclicGroup::identity())),
        permutation
            .corners()
            .act(&(CornerCubelet(BUL), CyclicGroup::identity())),
    ]);
    cmll.map(|cmll| {
        let mut cmll = cmll.clone();
        // Remove any trailing AUF move.
        if let Some(Move { axis: Axis::UD, .. }) = cmll.moves.last() {
            cmll.moves.pop();
        }
        cmll
    })
}

pub fn eolr(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    EOLR.solve(&(
        [
            permutation
                .edges()
                .unact(&(EdgeCubelet(UF), CyclicGroup::identity()))
                .1,
            permutation
                .edges()
                .unact(&(EdgeCubelet(UB), CyclicGroup::identity()))
                .1,
            permutation
                .edges()
                .unact(&(EdgeCubelet(DB), CyclicGroup::identity()))
                .1,
            permutation
                .edges()
                .unact(&(EdgeCubelet(DF), CyclicGroup::identity()))
                .1,
            permutation
                .edges()
                .unact(&(EdgeCubelet(UL), CyclicGroup::identity()))
                .1,
            permutation
                .edges()
                .unact(&(EdgeCubelet(UR), CyclicGroup::identity()))
                .1,
        ],
        [
            permutation
                .edges()
                .act(&(EdgeCubelet(UL), CyclicGroup::identity()))
                .0,
            permutation
                .edges()
                .act(&(EdgeCubelet(UR), CyclicGroup::identity()))
                .0,
        ],
        permutation
            .corners()
            .act(&(CornerCubelet(FUL), CyclicGroup::identity()))
            .0,
        matches!(
            permutation.centres().act(&CentreCubelet(FaceType::F)).0,
            FaceType::F | FaceType::B
        ),
    ))
}

pub fn l4e(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    L4E.solve(&(
        [
            permutation
                .edges()
                .act(&(EdgeCubelet(UF), CyclicGroup::identity()))
                .0,
            permutation
                .edges()
                .act(&(EdgeCubelet(UB), CyclicGroup::identity()))
                .0,
            permutation
                .edges()
                .act(&(EdgeCubelet(DB), CyclicGroup::identity()))
                .0,
            permutation
                .edges()
                .act(&(EdgeCubelet(DF), CyclicGroup::identity()))
                .0,
        ],
        permutation.centres().act(&CentreCubelet(FaceType::F)),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        group::{CyclicGroup, GroupAction, InverseSemigroup, Magma},
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
        let solution_first_edge = first_edge(permutation).unwrap();
        println!("First edge: {}", solution_first_edge);
        permutation =
            CubePermutation3::from_move_sequence(solution_first_edge.clone()).op(permutation);

        // Solve the first pair.
        let solution_first_pair = first_pair(permutation).unwrap();
        println!("First pair: {}", solution_first_pair);
        permutation =
            CubePermutation3::from_move_sequence(solution_first_pair.clone()).op(permutation);

        // Solve the second pair.
        let solution_second_pair = second_pair(permutation).unwrap();
        println!("Second pair: {}", solution_second_pair);
        permutation =
            CubePermutation3::from_move_sequence(solution_second_pair.clone()).op(permutation);

        // Solve the second edge.
        let solution_second_edge = second_edge(permutation).unwrap();
        println!("Second edge: {}", solution_second_edge);
        permutation =
            CubePermutation3::from_move_sequence(solution_second_edge.clone()).op(permutation);

        // Solve the third pair.
        let solution_third_pair = third_pair(permutation).unwrap();
        println!("Third pair: {}", solution_third_pair);
        permutation =
            CubePermutation3::from_move_sequence(solution_third_pair.clone()).op(permutation);

        // Solve the fourth pair.
        let solution_fourth_pair = fourth_pair(permutation).unwrap();
        println!("Fourth pair: {}", solution_fourth_pair);
        permutation =
            CubePermutation3::from_move_sequence(solution_fourth_pair.clone()).op(permutation);

        // Solve CMLL.
        let solution_cmll = cmll(permutation).unwrap();
        println!("CMLL: {}", solution_cmll);
        permutation = CubePermutation3::from_move_sequence(solution_cmll).op(permutation);

        // Solve EOLR.
        let solution_eolr = eolr(permutation).unwrap();
        println!("EOLR: {}", solution_eolr);
        permutation = CubePermutation3::from_move_sequence(solution_eolr.clone()).op(permutation);

        // Solve L4E.
        let solution_l4e = l4e(permutation).unwrap();
        println!("L4E: {}", solution_l4e);
        permutation = CubePermutation3::from_move_sequence(solution_l4e.clone()).op(permutation);

        assert_eq!(permutation, CubePermutation3::identity());
    }
}
