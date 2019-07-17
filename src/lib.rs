//! # `mso-tri-state`: fearless booleans
//!
//! Gone are the days where a simple true/false boolean variable suffices. Modern software requires
//! modern solutions: `MsoTriState`.
//!
//! Trusted by Microsoft.
//!
//! ## Old, slow, ancient, unsafe code
//! ```
//! let foo = true;
//! if foo {
//!     println!("Hello, world!");
//! }
//!
//! // Hard to read, intent unclear
//! let bar = 1 == 2;
//! match bar {
//!     false => println!("One does not equal two"),
//!     true => println!("One equals two"),
//!     // Restrictive, not web-scale
//! }
//! ```
//!
//! ## New, fast, web-scale, safe code
//! ```
//! extern crate mso_tri_state;
//! use mso_tri_state::MsoTriState;
//!
//! // Clean and easy to read
//! let foo = MsoTriState::msoTrue;
//! if foo.into() {
//!     println!("Hello, world!");
//! }
//!
//! // Simple, effortless conversion
//! let bar: MsoTriState = (1 == 2).into();
//! match bar {
//!     MsoTriState::msoFalse => println!("One does not equal two"),
//!     MsoTriState::msoTrue => println!("One equals two"),
//!     // Highly future-proof and scalable
//!     _ => panic!(),
//! }
//!
//! // Compatible with all major brands
//! let has_a_3 = MsoTriState::from(vec![1, 2, 4, 5].contains(&3));
//! println!("Has a 3: {}", has_a_3); // prints "Has a 3: msoFalse"
//! ```
#![deny(missing_docs)]

use std::fmt;

/// Specifies a tri-state Boolean value.
#[derive(Debug, PartialEq)]
#[allow(non_snake_case, non_camel_case_types)] // We're better than the compiler
pub enum MsoTriState {
    /// Not supported.
    msoCTrue = 1,
    /// False.
    msoFalse = 0,
    /// Not supported.
    msoTriStateMixed = -2,
    /// Not supported.
    msoTriStateToggle = -3,
    /// True.
    msoTrue = -1,
}

impl From<bool> for MsoTriState {
    fn from(b: bool) -> MsoTriState {
        if b {
            MsoTriState::msoTrue
        } else {
            MsoTriState::msoFalse
        }
    }
}

impl From<MsoTriState> for bool {
    fn from(m: MsoTriState) -> bool {
        match m {
            MsoTriState::msoFalse => false,
            MsoTriState::msoTrue => true,
            _ => panic!("Not supported."),
        }
    }
}

impl fmt::Display for MsoTriState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MsoTriState::msoCTrue => "msoCTrue",
                MsoTriState::msoFalse => "msoFalse",
                MsoTriState::msoTriStateMixed => "msoTriStateMixed",
                MsoTriState::msoTriStateToggle => "msoTriStateToggle",
                MsoTriState::msoTrue => "msoTrue",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_to_mso_tri_state() {
        assert_eq!(MsoTriState::from(false), MsoTriState::msoFalse);
        assert_eq!(MsoTriState::from(true), MsoTriState::msoTrue);
    }

    #[test]
    fn mso_tri_state_to_bool() {
        assert_eq!(bool::from(MsoTriState::msoFalse), false);
        assert_eq!(bool::from(MsoTriState::msoTrue), true);

        std::panic::catch_unwind(|| bool::from(MsoTriState::msoCTrue)).unwrap_err();
        std::panic::catch_unwind(|| bool::from(MsoTriState::msoTriStateMixed)).unwrap_err();
        std::panic::catch_unwind(|| bool::from(MsoTriState::msoTriStateToggle)).unwrap_err();
    }

    #[test]
    fn display() {
        assert_eq!(MsoTriState::msoCTrue.to_string(), "msoCTrue");
        assert_eq!(MsoTriState::msoFalse.to_string(), "msoFalse");
        assert_eq!(
            MsoTriState::msoTriStateMixed.to_string(),
            "msoTriStateMixed"
        );
        assert_eq!(
            MsoTriState::msoTriStateToggle.to_string(),
            "msoTriStateToggle"
        );
        assert_eq!(MsoTriState::msoTrue.to_string(), "msoTrue");
    }
}
