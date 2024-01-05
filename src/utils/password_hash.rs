use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn create_hash(password: &[u8]) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    match argon2.hash_password(password, &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err("Error: could not create a password hash.".to_string()),
    }
}

pub fn check_password(hash: &str, password: &[u8]) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password, &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}
