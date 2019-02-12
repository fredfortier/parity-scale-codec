// Copyright 2017, 2018 Parity Technologies
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

//! Serialiser and prepender.

use crate::Codec;
use core::iter::Extend;
use crate::alloc::vec::Vec;

/// Trait to allow itself to be serialised and prepended by a given slice.
pub trait KeyedVec {
	fn to_keyed_vec(&self, prepend_key: &[u8]) -> Vec<u8>;
}

impl<T: Codec> KeyedVec for T {
	fn to_keyed_vec(&self, prepend_key: &[u8]) -> Vec<u8> {
		self.using_encoded(|slice| {
			let mut r = prepend_key.to_vec();
			r.extend(slice);
			r
		})
	}
}
