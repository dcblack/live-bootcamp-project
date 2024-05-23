use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
  UserAlreadyExists,
  UserNotFound,
  InvalidCredentials,
  UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
  pub users: HashMap<String, User>,
}

impl HashmapUserStore {
  pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
    // Return `UserStoreError::UserAlreadyExists` if the user already exists,
    // otherwise insert the user into the hashmap and return `Ok(())`.
    if self.users.contains_key(&user.email) {
      Err(UserStoreError::UserAlreadyExists)
    } else {
      self.users.insert(user.email.clone(), user);
      Ok(())
    }
  }

  // Public method called `get_user` takes an
  // immutable reference to self and an email string slice as arguments.
  // This function should return a `Result` type containing either a
  // `User` object or a `UserStoreError`.
  // Return `UserStoreError::UserNotFound` if the user can not be found.
  pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
    let user = self.users.get(email);
    match user {
      Some(user) => Ok(user.clone()),
      None => Err(UserStoreError::UserNotFound)
    }
  }

  // TODO: Implement a public method called `validate_user`, which takes an
  // immutable reference to self, an email string slice, and a password string slice
  // as arguments. `validate_user` should return a `Result` type containing either a
  // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
  // Return `UserStoreError::UserNotFound` if the user can not be found.
  // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
  pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
    let user = self.users.get(email);
    match user {
      Some(user) => {
        if user.password == password {
          Ok(())
        } else {
          Err(UserStoreError::InvalidCredentials)
        }
      },
      _ => Err(UserStoreError::UserNotFound),
    }
  }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_user() {
    let mut store = HashmapUserStore::default();

    // Add first user
    let joe1 = User::new("joe@gmail.com".to_owned(), "xyzzy".to_owned(), false);
    let added = store.add_user(joe1);
    assert_eq!(added, Ok(()));

    // Add second user
    let tom = User::new("tom@gmail.com".to_owned(), "secret".to_owned(), true);
    let added = store.add_user(tom);
    assert_eq!(added, Ok(()));

    // Add pre-existing
    let joe2 = User::new("joe@gmail.com".to_owned(), "xyzzy".to_owned(), false);
    let added = store.add_user(joe2);
    assert_eq!(added, Err(UserStoreError::UserAlreadyExists));
  }

  #[test]
  fn test_get_user() {
    let mut store = HashmapUserStore::default();
    let joe = User::new("joe@gmail.com".to_owned(), "xyzzy".to_owned(), false);
    let added = store.add_user(joe.clone());
    let tom = User::new("tom@gmail.com".to_owned(), "secret".to_owned(), true);
    let added = store.add_user(tom.clone());

    let user = store.get_user("joe@gmail.com");
    assert_eq!(user, Ok(joe));
    let user = store.get_user("jane@gmail.com");
    assert_eq!(user, Err(UserStoreError::UserNotFound))
  }

  #[test]
  fn test_validate_user() {
    let mut store = HashmapUserStore::default();
    let joe = User::new("joe@gmail.com".to_owned(), "xyzzy".to_owned(), false);
    let added = store.add_user(joe);

    let attempt = store.validate_user("joe@gmail.com", "xyzzy");
    assert_eq!(attempt, Ok(()));

    let attempt = store.validate_user("Jon@gmail.com", "xyzzy");
    assert_eq!(attempt, Err(UserStoreError::UserNotFound));

    let attempt = store.validate_user("joe@gmail.com", "xyzz!");
    assert_eq!(attempt, Err(UserStoreError::InvalidCredentials));
  }
}
