use crate::{
    algorithmic::AlgorithmicSolver,
    cube::{
        Axis, FaceType, Move, MoveSequence,
        {CornerType::*, EdgeType::*},
    },
    group::{CyclicGroup, GroupAction, Magma, Unital},
    intuitive::{SequenceGraph, SequenceSolver},
    permute::{CentreCubelet, CornerCubelet, CubePermutation3, EdgeCubelet},
    solve::{move_sequence_to_intuitive_action, Action, ActionReason, ActionSteps},
};

type RouxEdgeSignature = (EdgeCubelet, CyclicGroup<2>);
type RouxPairSignature = (
    (EdgeCubelet, CyclicGroup<2>),
    (CornerCubelet, CyclicGroup<3>),
);
type RouxCmllSignature = [(CornerCubelet, CyclicGroup<3>); 4];
type RouxEoSignature = ([CyclicGroup<2>; 6], bool);
type RouxLrSignature = ([EdgeCubelet; 2], CornerCubelet);
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

    static ref CMLL: AlgorithmicSolver<RouxCmllSignature> = {
        // Kian's CMLL algs:
        let alg_set = vec![
            // O
            "R U R' F' R U R' U' R' F R2 U' R'",
            "F R U' R' U' R U R' F' R U R' U' R' F R F'",
            // H
            "R U2 R' U' R U R' U' R U' R'",
            "F R U R' U' R U R' U' R U R' U' F'",
            "R U2' R2' F R F' U2 R' F R F'",
            "r U' r2' D' r U' r' D r2 U r'",
            // Pi
            "F R U R' U' R U R' U' F'",
            "F R' F' R U2 R U' R' U R U2' R'",
            "R' F R U F U' R U R' U' F'",
            "R U2 R' U' R U R' U2' R' F R F'",
            "r U' r2' D' r U r' D r2 U r'",
            "R' U' R' F R F' R U' R' U2 R",
            // U
            "R2 D R' U2 R D' R' U2 R'",
            "R2' D' R U2 R' D R U2 R",
            "R2' F U' F U F2 R2 U' R' F R",
            "F R2 D R' U R D' R2' U' F'",
            "r U' r' U r' D' r U' r' D r",
            "F R U R' U' F'",
            // T
            "R U R' U' R' F R F'",
            "L' U' L U L F' L' F",
            "F R' F R2 U' R' U' R U R' F2",
            "r' U r U2' R2' F R F' R",
            "r' D' r U r' D r U' r U r'",
            "r2' D' r U r' D r2 U' r' U' r",
            // Sune
            "R U R' U R U2 R'",
            "L' U2 L U2' L F' L' F",
            "F R' F' R U2 R U2' R'",
            "R U R' U' R' F R F' R U R' U R U2' R'",
            "R U R' U R' F R F' R U2' R'",
            "R U' L' U R' U' L",
            // Antisune
            "R' U' R U' R' U2' R",
            "R2 D R' U R D' R' U R' U' R U' R'",
            "F' L F L' U2' L' U2 L",
            "R U2' R' U2 R' F R F'",
            "L' U R U' L U R'",
            "R' U' R U' R' U R' F R F' U R",
            ]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let pre_moves = vec!["U".parse::<MoveSequence>().unwrap()];
        let post_moves = vec!["U".parse::<MoveSequence>().unwrap()];

        AlgorithmicSolver::new("roux_cmll", alg_set, pre_moves, post_moves, |cube| {
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
        }, |seq| {
            seq.moves.len() as u64
        })
    };

    static ref EO: SequenceSolver<RouxEoSignature> = {
        let gen_set = vec!["U", "M"]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new("roux_eo", gen_set, |cube| {
            let axis_swapped = matches!(cube.centres().act(&CentreCubelet(FaceType::U)).0, FaceType::F | FaceType::B);

            ([
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
            ], axis_swapped)
        });
        graph.search(([CyclicGroup::identity(); 6], false), |seq| {
            seq.moves.len() as u64
        })
    };

    /// The cube is assumed to have U/D faces pointing on U/D (or swapped).
    static ref LR: SequenceSolver<RouxLrSignature> = {
        let gen_set = vec![
            "U", "M2",
            "M U2 M", "M U2 M'", "M' U2 M'", "M' U2 M",
        ]
            .into_iter()
            .map(|x| x.parse::<MoveSequence>().unwrap())
            .collect::<Vec<_>>();

        let graph = SequenceGraph::new("roux_lr", gen_set, |cube| {
            (
                [
                    cube.edges()
                        .act(&(EdgeCubelet(UL), CyclicGroup::identity())).0,
                    cube.edges()
                        .act(&(EdgeCubelet(UR), CyclicGroup::identity())).0,
                ],
                cube.corners()
                    .act(&(CornerCubelet(FUL), CyclicGroup::identity())).0,
            )
        });
        graph.search(([EdgeCubelet(UL), EdgeCubelet(UR)], CornerCubelet(FUL)), |seq| {
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
        // There must be an even number of U2 mnves,
        // otherwise LR is messed up.
        // However, the L4E are actually unsolvable wih an odd amount of U2 moves, since
        // a 2-cycle of edges is impossible excluding U2 moves!
        // So somewhat surprisingly, this signature is sufficient.
        let gen_set = vec![
            "M", "U2", "E2 M' E2", "E2 M E2"
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

pub fn first_edge_action(permutation: CubePermutation3) -> Option<Action> {
    first_edge(permutation).map(|seq| move_sequence_to_intuitive_action("First edge", seq.clone()))
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

pub fn first_pair_action(permutation: CubePermutation3) -> Option<Action> {
    first_pair(permutation).map(|seq| move_sequence_to_intuitive_action("First pair", seq.clone()))
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

pub fn second_pair_action(permutation: CubePermutation3) -> Option<Action> {
    second_pair(permutation)
        .map(|seq| move_sequence_to_intuitive_action("Second pair", seq.clone()))
}

pub fn second_edge(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    ROUX_SECOND_EDGE.solve(
        &permutation
            .edges()
            .act(&(EdgeCubelet(DR), CyclicGroup::identity())),
    )
}

pub fn second_edge_action(permutation: CubePermutation3) -> Option<Action> {
    second_edge(permutation)
        .map(|seq| move_sequence_to_intuitive_action("Second edge", seq.clone()))
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

pub fn third_pair_action(permutation: CubePermutation3) -> Option<Action> {
    third_pair(permutation).map(|seq| move_sequence_to_intuitive_action("Third pair", seq.clone()))
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

pub fn fourth_pair_action(permutation: CubePermutation3) -> Option<Action> {
    fourth_pair(permutation)
        .map(|seq| move_sequence_to_intuitive_action("Fourth pair", seq.clone()))
}

pub fn cmll(permutation: CubePermutation3) -> Option<MoveSequence> {
    let cmll = CMLL.solve(&[
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

pub fn cmll_action(permutation: CubePermutation3) -> Option<Action> {
    cmll(permutation).map(|seq| move_sequence_to_intuitive_action("CMLL", seq))
}

pub fn eo(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    let axis_swapped = matches!(
        permutation.centres().act(&CentreCubelet(FaceType::U)).0,
        FaceType::F | FaceType::B
    );

    EO.solve(&(
        [
            // Unact is used to get edge orientation: we don't care which edge is in this position,
            // just how it is oriented relative to where the edge should belong.
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
        axis_swapped,
    ))
}

pub fn eo_action(permutation: CubePermutation3) -> Option<Action> {
    eo(permutation)
        .map(|seq| move_sequence_to_intuitive_action("Orientation of last six edges", seq.clone()))
}

pub fn lr(permutation: CubePermutation3) -> Option<&'static MoveSequence> {
    LR.solve(&(
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
    ))
}

pub fn lr_action(permutation: CubePermutation3) -> Option<Action> {
    lr(permutation).map(|seq| move_sequence_to_intuitive_action("UL and UR edges", seq.clone()))
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

pub fn eolr_action(permutation: CubePermutation3) -> Option<Action> {
    eolr(permutation).map(|seq| move_sequence_to_intuitive_action("EOLR", seq.clone()))
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

pub fn l4e_action(permutation: CubePermutation3) -> Option<Action> {
    l4e(permutation).map(|seq| move_sequence_to_intuitive_action("Last four edges", seq.clone()))
}

pub fn solve(mut permutation: CubePermutation3) -> Option<Action> {
    let mut steps = Vec::new();

    // Can't use impl FnOnce or anything, so just use fn.
    let mut add_step = |func: fn(CubePermutation3) -> Option<Action>| -> Option<()> {
        let step = func(permutation)?;
        permutation =
            CubePermutation3::from_move_sequence(step.steps.move_sequence()).op(permutation);
        steps.push(step);
        Some(())
    };

    add_step(first_edge_action);
    add_step(first_pair_action);
    add_step(second_pair_action);
    add_step(second_edge_action);
    add_step(third_pair_action);
    add_step(fourth_pair_action);
    add_step(cmll_action);
    add_step(eo_action);
    add_step(lr_action);
    add_step(l4e_action);

    Some(Action {
        reason: ActionReason::Solve,
        description: Some("Roux method".to_string()),
        steps: ActionSteps::Sequence { actions: steps },
    })
}

#[cfg(test)]
mod tests {
    use crate::{
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

        let permutation = CubePermutation3::from_move_sequence(scramble);
        let solution = solve(permutation).unwrap();
        println!("Solution: {:#?}", solution);
        let final_permutation =
            CubePermutation3::from_move_sequence(solution.steps.move_sequence()).op(permutation);

        assert_eq!(final_permutation, CubePermutation3::identity());
    }
}
