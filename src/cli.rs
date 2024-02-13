use crate::state::State;

pub fn print_state(state: &State) {
    dbg!(state.available_locations());
}
