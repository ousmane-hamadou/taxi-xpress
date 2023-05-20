use bcrypt::BcryptResult;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::tokio::task;

const DEFAULT_PASSWORD_LENGTH: usize = 8;

pub async fn hash(password: String) -> BcryptResult<String> {
    task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST))
        .await
        .unwrap()
}

pub async fn verify(password: &str, hash: &str) -> BcryptResult<bool> {
    let hash = hash.to_owned();
    let password = password.to_owned();

    task::spawn_blocking(move || bcrypt::verify(password.to_owned(), &hash))
        .await
        .unwrap()
}

pub struct RandPassword {
    pub value: String,
    pub hashed: String,
}

/* FIXME: delete fake_password */
pub async fn gen_password() -> BcryptResult<RandPassword> {
    let _password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(DEFAULT_PASSWORD_LENGTH)
        .map(char::from)
        .collect();

    let fake_password = String::from("hello_world");
    let hashed_password = hash(fake_password.clone()).await?;

    Ok(RandPassword {
        value: fake_password,
        hashed: hashed_password,
    })
}
