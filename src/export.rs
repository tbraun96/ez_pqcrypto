use crate::PostQuantumContainer;

/// The default type to store data from a [PostQuantumContainer]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PostQuantumExport {
    pub(super) algorithm: u8,
    pub(super) public_key: Vec<u8>,
    pub(super) secret_key: Option<Vec<u8>>,
    pub(super) ciphertext: Option<Vec<u8>>,
    pub(super) shared_secret: Option<Vec<u8>>
}

impl From<&'_ PostQuantumContainer> for PostQuantumExport {
    fn from(container: &PostQuantumContainer) -> Self {
        let algorithm = container.algorithm;
        let public_key = container.get_public_key().to_vec();
        let secret_key = {
            if let Ok(secret_key) = container.get_secret_key() {
                Some(secret_key.to_vec())
            } else {
                None
            }
        };

        let ciphertext = {
            if let Ok(ciphertext) = container.get_ciphertext() {
                Some(ciphertext.to_vec())
            } else {
                None
            }
        };

        let shared_secret = {
            if let Ok(shared_secret) = container.get_shared_secret() {
                Some(shared_secret.to_vec())
            } else {
                None
            }
        };

        Self { algorithm, public_key, secret_key, ciphertext, shared_secret }
    }
}