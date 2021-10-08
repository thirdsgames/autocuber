use crate::{cube::MoveSequence, permute::CubePermutation3, Move};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

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
    /// TODO: Moves can be cancelled into other moves.
    /// We should be able to mark moves as "cancelled" so that
    /// they appear but do not ever get performed or contribute to move count.
    Move { mv: Move },
    /// Perform this sequence of actions.
    Sequence { actions: Vec<Action> },
}

impl ActionSteps {
    pub fn move_sequence(&self) -> MoveSequence {
        match self {
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

#[wasm_bindgen]
#[allow(dead_code)]
pub fn action_to_div() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let history = document.get_element_by_id("history-action").unwrap();

    let action = crate::roux::solve(CubePermutation3::from_move_sequence(
        "U2 B D' B U2 L F' D B' U2 D R' U2 B R2 D' B' D2 L B2 F2 U D2 F B2"
            .parse()
            .unwrap(),
    ))
    .unwrap();

    // Clear the history div.
    let range = document.create_range().unwrap();
    range.select_node_contents(&history).unwrap();
    range.delete_contents().unwrap();

    add_action_to_div(action, &document, &history).unwrap();
}

fn add_action_to_div(action: Action, document: &Document, div: &Element) -> Result<(), JsValue> {
    let reason = match &action.reason {
        ActionReason::Solve => Some("Solve the cube".to_string()),
        ActionReason::SolveStep { step_name } => Some(step_name.to_string()),
        ActionReason::Intuitive => None,
    };

    let val = document.create_element("p")?;
    if let Some(mut reason) = reason {
        if action.description.is_some() {
            reason += ": ";
        }
        val.set_text_content(Some(&reason));
    }
    if let Some(description) = action.description {
        let i = document.create_element("i")?;
        i.set_text_content(Some(&description));
        val.append_child(&i)?;
    }
    div.append_child(&val)?;

    match action.steps {
        ActionSteps::Move { mv } => {
            let p = document.create_element("p")?;
            p.set_text_content(Some(&mv.to_string()));
            div.append_child(&p)?;
        }
        ActionSteps::Sequence { actions } => {
            let list = document.create_element(match &action.reason {
                ActionReason::Solve => "ol",
                _ => "ul",
            })?;
            // For each action that's just a move with no description, collate them into this list.
            let mut collated_moves = Vec::new();
            for sub_action in actions {
                match &sub_action {
                    Action {
                        reason: _,
                        description: None,
                        steps: ActionSteps::Move { mv },
                    } => {
                        collated_moves.push(*mv);
                    }
                    _ => {
                        // It's not just a simple move.
                        // Add it as a bullet point.
                        // But first, add the collated moves.
                        if !collated_moves.is_empty() {
                            let li = document.create_element("li")?;
                            li.set_text_content(Some(
                                &MoveSequence {
                                    moves: std::mem::take(&mut collated_moves),
                                }
                                .to_string(),
                            ));
                            list.append_child(&li)?;
                        }

                        let li = document.create_element("li")?;
                        add_action_to_div(sub_action, document, &li)?;
                        list.append_child(&li)?;
                    }
                }
            }
            if !collated_moves.is_empty() {
                let li = document.create_element("li")?;
                li.set_text_content(Some(
                    &MoveSequence {
                        moves: std::mem::take(&mut collated_moves),
                    }
                    .to_string(),
                ));
                list.append_child(&li)?;
            }
            div.append_child(&list)?;
        }
    }

    Ok(())
}
