/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};

/// The different kinds of errors that can be returned
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
#[non_exhaustive]
pub enum RustbreakErrorKind {
    /// A context error when a serialization failed
    #[fail(display = "Could not serialize the value")]
    Serialization,
    /// A context error when a deserialization failed
    #[fail(display = "Could not deserialize the value")]
    Deserialization,
    /// This error is returned if the `Database` is poisoned. See
    /// `Database::write` for details
    #[fail(display = "The database has been poisoned")]
    Poison,
    /// An error in the backend happened
    #[fail(display = "The backend has encountered an error")]
    Backend,
    /// If `Database::write_safe` is used and the closure panics, this error is
    /// returned
    #[fail(display = "The write operation paniced but got caught")]
    WritePanic,
}

/// The main error type that gets returned for errors that happen while
/// interacting with a `Database`.
#[derive(Debug)]
pub struct RustbreakError {
    inner: Context<RustbreakErrorKind>,
}

impl Fail for RustbreakError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for RustbreakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl RustbreakError {
    /// Get the kind of this error
    pub fn kind(&self) -> RustbreakErrorKind {
        *self.inner.get_context()
    }
}

impl From<RustbreakErrorKind> for RustbreakError {
    fn from(kind: RustbreakErrorKind) -> Self {
        Self {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<RustbreakErrorKind>> for RustbreakError {
    fn from(inner: Context<RustbreakErrorKind>) -> Self {
        Self { inner }
    }
}

/// A simple type alias for errors
pub type Result<T> = std::result::Result<T, RustbreakError>;

#[cfg(test)]
mod tests {
    use super::{RustbreakError, RustbreakErrorKind};
    use failure::Context;
    use std::any::Any;

    #[test]
    fn static_errorkind_impl_any() {
        let err = RustbreakErrorKind::Backend;
        let boxed: Box<dyn Any> = Box::new(err);
        assert!(boxed.is::<RustbreakErrorKind>());
    }

    #[test]
    fn static_error_impl_any() {
        let context = RustbreakErrorKind::Serialization;
        let err: RustbreakError = Context::new(context).into();
        let boxed: Box<dyn Any> = Box::new(err);
        assert!(boxed.is::<RustbreakError>());
    }
}
