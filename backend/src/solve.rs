use crate::Move;

/// An action is something you can do on a cube,
/// and that you have a reason for doing.
pub struct Action {
    /// Why (at a base level) did we do this action?
    reason: ActionReason,
    /// What other arbitrary information do we have about why we did this action?
    description: Option<String>,
    /// What steps must we perform to execute this action?
    steps: ActionSteps,
}

pub enum ActionReason {
    /// This action was one step in a solve method.
    SolveStep { step_name: String },
}

/// TODO: Add conjugate, commutator, and algorithmic action steps.
pub enum ActionSteps {
    /// TODO: Moves can be cancelled into other moves.
    /// We should be able to mark moves as "cancelled" so that
    /// they appear but do not ever get performed or contribute to move count.
    Move { mv: Move },
    /// Perform this sequence of actions.
    Sequence { actions: Vec<Action> },
}
