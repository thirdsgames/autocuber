use crate::{cube::MoveSequence, Move};

/// An action is something you can do on a cube,
/// and that you have a reason for doing.
#[derive(Debug)]
pub struct Action {
    /// Why (at a base level) did we do this action?
    pub reason: ActionReason,
    /// What other arbitrary information do we have about why we did this action?
    pub description: Option<String>,
    /// What steps must we perform to execute this action?
    pub steps: ActionSteps,
}

#[derive(Debug)]
pub enum ActionReason {
    /// This action was a full solve.
    Solve,
    /// This action was one step in a solve method.
    SolveStep { step_name: &'static str },
    /// This action was performed intuitively
    Intuitive,
}

/// TODO: Add conjugate, commutator, and algorithmic action steps.
#[derive(Debug)]
pub enum ActionSteps {
    Nothing,
    /// TODO: Moves can be cancelled into other moves.
    /// We should be able to mark moves as "cancelled" so that
    /// they appear but do not ever get performed or contribute to move count.
    Move {
        mv: Move,
    },
    /// Perform this sequence of actions.
    Sequence {
        actions: Vec<Action>,
    },
}

impl ActionSteps {
    pub fn move_sequence(&self) -> MoveSequence {
        match self {
            ActionSteps::Nothing => MoveSequence { moves: Vec::new() },
            ActionSteps::Move { mv } => MoveSequence { moves: vec![*mv] },
            ActionSteps::Sequence { actions } => MoveSequence {
                moves: actions
                    .iter()
                    .map(|act| act.steps.move_sequence().moves)
                    .flatten()
                    .collect(),
            },
        }
    }
}
