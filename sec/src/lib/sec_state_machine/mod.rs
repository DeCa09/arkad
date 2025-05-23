use sec_state::SecState;
pub use state_maschine::prelude::*;

pub mod extract;
pub mod sec_context_data;
pub mod sec_error;
pub mod sec_state;
pub mod sec_state_data;
pub mod sec_super_state;
pub mod sec_transition;

pub trait SecStateMachine<S>: StateMachine<S>
where
    S: SecState,
{
}
