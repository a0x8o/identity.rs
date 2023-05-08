// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A wrapper around a JSON Web Token (JWK).
pub struct Jwt(String);

impl Jwt {
  /// Creates a new `Jwt`.
  pub fn new(jwt_string: String) -> Self {
    Self(jwt_string)
  }

  /// Returns a reference of the JWT string.
  pub fn as_string(&self) -> &String {
    &self.0
  }
}

impl From<String> for Jwt {
  fn from(jwt: String) -> Self {
    Self::new(jwt)
  }
}
impl From<Jwt> for String {
  fn from(jwt: Jwt) -> Self {
    jwt.0
  }
}