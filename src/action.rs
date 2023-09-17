#[derive(PartialEq, Copy, Clone)]
pub enum Action {
    Hit,
    Stand,
    Double,
    Split,
    Surrender,
}

pub struct PossibleAction(pub(crate) Action);

impl PossibleAction {
    pub fn action(&self) -> Action {
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

#[test]
fn possible_action_action_partial_eq() {
    let possible_action = &&&PossibleAction(Action::Hit);
    assert!(possible_action == Action::Hit);
}
