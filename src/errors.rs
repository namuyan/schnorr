// Copyright 2018 Stichting Organism
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// -*- mode: rust; -*-
//
// This file is part of ed25519-dalek.
// Copyright (c) 2017 Isis Lovecruft
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>

//! Errors which may occur when parsing keys and/or signatures to or from wire formats.

// rustc seems to think the typenames in match statements (e.g. in
// Display) should be snake cased, for some reason.
#![allow(non_snake_case)]

use core::fmt;
use core::fmt::Display;

/// Internal errors.  Most application-level developers will likely not
/// need to pay any attention to these.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub (crate) enum InternalError {
    PointDecompressionError,
    ScalarFormatError,
    /// An error in the length of bytes handed to a constructor.
    ///
    /// To use this, pass a string specifying the `name` of the type which is
    /// returning the error, and the `length` in bytes which its constructor
    /// expects.
    BytesLengthError{ name: &'static str, length: usize },
    /// The verification equation wasn't satisfied
    VerifyError,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InternalError::PointDecompressionError
                => write!(f, "Cannot decompress Edwards point"),
            InternalError::ScalarFormatError
                => write!(f, "Cannot use scalar with high-bit set"),
            InternalError::BytesLengthError{ name: n, length: l}
                => write!(f, "{} must be {} bytes in length", n, l),
            InternalError::VerifyError
                => write!(f, "Verification equation was not satisfied"),
        }
    }
}

impl ::failure::Fail for InternalError {}

/// Errors which may occur while processing signatures and keypairs.
///
/// This error may arise due to:
///
/// * Being given bytes with a length different to what was expected.
///
/// * A problem decompressing `r`, a curve point, in the `Signature`, or the
///   curve point for a `PublicKey`.
///
/// * A problem with the format of `s`, a scalar, in the `Signature`.  This
///   is only raised if the high-bit of the scalar was set.  (Scalars must
///   only be constructed from 255-bit integers.)
///
/// * Failure of a signature to satisfy the verification equation.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct SchnorrError(pub (crate) InternalError);

impl Display for SchnorrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ::failure::Fail for SchnorrError {
    fn cause(&self) -> Option<&::failure::Fail> {
        Some(&self.0)
    }
}