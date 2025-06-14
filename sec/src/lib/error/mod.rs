//! # Error Handling for SEC State Machine Library
//!
//! This module defines the core error types and conversions for the SEC state machine framework.
//! It provides a strongly-typed, extensible error hierarchy for state machine operations, including
//! errors originating from states, transitions, and the state machine itself.
//!
//! ## Modules
//! - [`state_machine`]: Contains error enums and types for state machine, state, and transition errors.
//!
//! ## Types
//! - [`ErrorKind`]: The top-level error enum representing all error kinds in the SEC state machine library.
//!   Supports downcasting to more specific error types for granular error handling.
//!
//! ## Usage
//! Use [`ErrorKind`] for error propagation and pattern matching in workflows that involve state machine processing.
//! Downcasting is supported via `TryInto` for extracting specific error variants (e.g., state or transition errors).
//!
//! ## Example
//! ```rust
//! use sec::error::{ErrorKind, StateMachine};
//! let err = ErrorKind::StateMachine(StateMachine::InvalidConfiguration);
//! match err {
//!     ErrorKind::StateMachine(sm_err) => println!("State machine error: {sm_err}"),
//!     ErrorKind::DowncastNotPossible => println!("Downcast failed"),
//!      _ => println!("Other error kind"),
//! }
//! ```

/// Error module for state machine operations.
pub mod state_machine;

pub use state_machine::StateMachine;
pub use state_machine::state::State;
pub use state_machine::transition::Transition;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
/// The top-level error enum for all SEC state machine errors.
///
/// This enum encapsulates errors from state machines, states, and transitions,
/// and provides conversions and downcasting for granular error handling.
pub enum ErrorKind {
    /// State machine related error.
    StateMachine(StateMachine),

    /// Error indicating that casting from [`ErrorKind`] to a more specific error type is not possible.
    DowncastNotPossible,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateMachine(state_machine) => {
                write!(
                    f,
                    "Problem occured during state machine execution: '{state_machine}'"
                )
            }
            Self::DowncastNotPossible => {
                write!(
                    f,
                    "Failed to downcast error of type `ErrorKind` into more specific error."
                )
            }
        }
    }
}

impl std::error::Error for ErrorKind {}

impl From<StateMachine> for ErrorKind {
    /// Converts a [`StateMachine`] error into an [`ErrorKind`].
    ///
    /// This allows seamless propagation of state machine errors as the top-level error type.
    fn from(error: StateMachine) -> Self {
        Self::StateMachine(error)
    }
}

impl TryFrom<ErrorKind> for StateMachine {
    type Error = ErrorKind;

    /// Attempts to convert an [`ErrorKind`] into a [`StateMachine`] error.
    ///
    /// Returns `Ok(StateMachine)` if the variant matches, or `Err(ErrorKind::DowncastNotPossible)` otherwise.
    fn try_from(value: ErrorKind) -> Result<Self, Self::Error> {
        match value {
            ErrorKind::StateMachine(state_machine) => Ok(state_machine),
            _ => Err(ErrorKind::DowncastNotPossible),
        }
    }
}

impl TryFrom<ErrorKind> for State {
    type Error = ErrorKind;

    /// Attempts to convert an [`ErrorKind`] into a [`State`] error.
    ///
    /// Returns `Ok(State)` if the variant matches, or `Err(ErrorKind::DowncastNotPossible)` otherwise.
    fn try_from(value: ErrorKind) -> Result<Self, Self::Error> {
        match value {
            ErrorKind::StateMachine(sm) => match sm {
                StateMachine::State(state) => Ok(state),
                StateMachine::Transition(_) | StateMachine::InvalidConfiguration => {
                    Err(ErrorKind::DowncastNotPossible)
                }
            },
            _ => Err(ErrorKind::DowncastNotPossible),
        }
    }
}

impl TryFrom<ErrorKind> for Transition {
    type Error = ErrorKind;

    /// Attempts to convert an [`ErrorKind`] into a [`Transition`] error.
    ///
    /// Returns `Ok(Transition)` if the variant matches, or `Err(ErrorKind::DowncastNotPossible)` otherwise.
    fn try_from(value: ErrorKind) -> Result<Self, Self::Error> {
        match value {
            ErrorKind::StateMachine(sm) => match sm {
                StateMachine::Transition(transition) => Ok(transition),
                StateMachine::State(_) | StateMachine::InvalidConfiguration => {
                    Err(ErrorKind::DowncastNotPossible)
                }
            },
            _ => Err(ErrorKind::DowncastNotPossible),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::{fmt::Debug, hash::Hash};

    const fn implements_auto_traits<T: Sized + Send + Sync + Unpin>() {}
    #[test]
    const fn should_be_able_to_rely_auto_trait_implementation_when_using_errorkind() {
        implements_auto_traits::<ErrorKind>();
    }

    const fn implements_send<T: Send>() {}
    const fn implements_sync<T: Sync>() {}

    #[test]
    const fn should_have_implementend_send_when_using_errorkind() {
        implements_send::<ErrorKind>();
    }

    #[test]
    const fn should_implement_sync_when_using_errorkind() {
        implements_sync::<ErrorKind>();
    }

    #[test]
    const fn should_be_thread_safe_when_using_errorkind() {
        implements_send::<ErrorKind>();
        implements_sync::<ErrorKind>();
    }

    const fn implements_sized<T: Sized>() {}
    #[test]
    const fn should_be_able_to_rely_on_error_being_sized_when_using_errorkind() {
        implements_sized::<ErrorKind>();
    }

    const fn implements_hash<T: Hash>() {}
    #[test]
    const fn should_be_able_to_rely_on_hash_implementation_when_using_errorkind() {
        implements_hash::<ErrorKind>();
    }

    const fn implements_partial_eq<T: PartialEq>() {}
    #[test]
    const fn should_be_able_to_rely_on_partial_eq_implementation_when_using_errorkind() {
        implements_partial_eq::<ErrorKind>();
    }

    const fn implements_eq<T: Eq>() {}
    #[test]
    const fn should_be_able_to_rely_on_eq_implementation_when_using_errorkind() {
        implements_eq::<ErrorKind>();
    }

    const fn implements_partial_ord<T: PartialOrd>() {}
    #[test]
    const fn should_be_able_to_rely_on_partial_ord_implementation_when_using_errorkind() {
        implements_partial_ord::<ErrorKind>();
    }

    const fn implements_ord<T: Ord>() {}
    #[test]
    const fn should_be_able_to_rely_on_ord_implementation_when_using_errorkind() {
        implements_ord::<ErrorKind>();
    }

    const fn implements_debug<T: Debug>() {}
    #[test]
    const fn should_be_able_to_rely_on_debug_implementation_when_using_errorkind() {
        implements_debug::<ErrorKind>();
    }

    const fn implements_clone<T: Clone>() {}
    #[test]
    const fn should_be_able_to_rely_on_clone_implementation_when_using_errorkind() {
        implements_clone::<ErrorKind>();
    }

    const fn implements_unpin<T: Unpin>() {}
    #[test]
    const fn should_be_able_to_rely_on_unpin_implementation_when_using_errorkind() {
        implements_unpin::<ErrorKind>();
    }

    #[test]
    fn should_be_able_to_create_state_machine_error_when_casting_from_specific_error_kind() {
        let _result: StateMachine = ErrorKind::StateMachine(StateMachine::InvalidConfiguration)
            .try_into()
            .expect(
                "Should always be able to cast harcdcoded `ErrorKind` into `StateMachine` error.",
            );
    }

    #[test]
    fn should_be_able_to_create_state_error_when_casting_from_specific_error_kind_that_is_a_state()
    {
        let _result: State = ErrorKind::StateMachine(StateMachine::State(State::InvalidInputData))
            .try_into()
            .expect(
                "Should always be able to cast provided harcdcoded `ErrorKind` into `State` error.",
            );
    }

    #[test]
    fn should_be_able_to_create_transition_error_when_casting_from_specific_error_kind_that_is_a_transition()
     {
        let _result: Transition =
            ErrorKind::StateMachine(StateMachine::Transition(Transition::FailedOutputConversion))
                .try_into()
                .expect(
                    "Should always be able to cast provided harcdcoded `ErrorKind` into `Transition` error.",
                );
    }

    #[test]
    fn should_be_able_to_create_state_machine_error_when_using_enum_directly() {
        let _result = StateMachine::InvalidConfiguration;
    }

    #[test]
    fn should_be_able_to_cast_into_equivalent_errorkind_error_when_having_a_statemachine_error() {
        let expected_result = ErrorKind::StateMachine(StateMachine::InvalidConfiguration);

        let result: ErrorKind = StateMachine::InvalidConfiguration.into();

        assert_eq!(result, expected_result);
    }
}
