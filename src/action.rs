#[derive(PartialEq, Copy, Clone)]
pub enum Action {
    // Deal(f64),
    Hit,
    Stand,
    Double,
    Split,
    Surrender,
}

// impl PartialEq for Action {
//     fn eq(&self, other: &Self) -> bool {
//         std::mem::discriminant(self) == std::mem::discriminant(other)
//     }
// }

pub struct PossibleAction(pub(crate) Action);

impl PossibleAction {
    pub fn action(&self) -> Action{
        self.0
    }
}

impl PartialEq<Action> for PossibleAction {
    fn eq(&self, other: &Action) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Action> for &PossibleAction {
    fn eq(&self, other: &Action) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Action> for &&PossibleAction {
    fn eq(&self, other: &Action) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Action> for &&&PossibleAction {
    fn eq(&self, other: &Action) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&Action> for PossibleAction {
    fn eq(&self, other: &&Action) -> bool {
        self.0 == **other
    }
}

// #[test]
// fn action_partial_eq() {
//     let action = Action::Deal(20.0);
//     let other_action = Action::Deal(0.0);
//     assert!(action == other_action);
// }

#[test]
fn possible_action_action_partial_eq() {
    let possible_action = &&&PossibleAction(Action::Hit);
    assert!(possible_action == Action::Hit);
}


