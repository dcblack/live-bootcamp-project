#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
  pub fn parse(s: String) -> Result<Password, String> {
    if validate_password(&s) {
      Ok(Self(s))
    } else {
      Err("Failed to parse string to a Password type".to_owned())
    }
  }
}

const MIN_PASSWORD_LENGTH: usize = 12;
fn validate_password(s: &str) -> bool {
  s.len() >= MIN_PASSWORD_LENGTH
    && s.chars().any(|c| c.is_digit(10))
    && s.chars().any(|c| c.is_uppercase())
    && s.chars().any(|c| c.is_lowercase())
    && s.chars().any(|c| c.is_ascii_punctuation())
}

impl AsRef<str> for Password {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
  use super::Password;

  use fake::faker::internet::en::Password as FakePassword;
  use fake::Fake;

  #[test]
  fn empty_string_is_rejected() {
    let password = "".to_owned();
    assert!(Password::parse(password).is_err());
  }

  const SHORT_PASSWORD: &str = "_bCdef$12ee"; // with valid characters
  const SPECIAL_CHARS: &str = "Li!1";

  #[test]
  fn string_less_than_min_characters_is_rejected() {
    let password = SHORT_PASSWORD.to_owned();
    assert!(Password::parse(password).is_err());
  }

  #[derive(Debug, Clone)]
  struct ValidPasswordFixture(pub String);

  impl quickcheck::Arbitrary for ValidPasswordFixture {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
      let mut password: String = FakePassword(12..28).fake_with_rng(g);
      password.push_str(SPECIAL_CHARS); // force special characters
      Self(password)
    }
  }
  #[quickcheck_macros::quickcheck]
  fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
    Password::parse(valid_password.0).is_ok()
  }
}
