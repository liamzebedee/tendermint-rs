use rand::rngs::OsRng;
use secp256k1::{ecdsa::SerializedSignature, Message, Secp256k1, SecretKey};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha3::{Digest, Keccak256};
use std::{
    fmt,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Debug, Copy)]
pub struct Signature(secp256k1::ecdsa::SerializedSignature);

#[derive(Clone, Debug, Copy)]
pub struct PublicKey(secp256k1::PublicKey);

pub type Keypair = ECDSAKeypair;

#[derive(Debug)]
pub struct ECDSAKeypair {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl Default for ECDSAKeypair {
    fn default() -> Self {
        Self::new()
    }
}

impl ECDSAKeypair {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        ECDSAKeypair { secret_key, public_key: PublicKey(public_key) }
    }

    pub fn new_from_privatekey(private_key: &str) -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_str(private_key).unwrap();

        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        ECDSAKeypair { secret_key, public_key: PublicKey(public_key) }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn get_secret_key(&self) -> SecretKey {
        self.secret_key
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let message = Message::from_slice(&hash).expect("32 bytes");
        let signature = secp.sign_ecdsa(&message, &self.secret_key);
        Signature(signature.serialize_der())
    }
}

pub fn verify_signature(
    data: &[u8],
    signature: &SerializedSignature,
    public_key: PublicKey,
) -> bool {
    let secp: Secp256k1<secp256k1::All> = Secp256k1::new();
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let message = Message::from_slice(&hash).expect("32 bytes");
    secp.verify_ecdsa(&message, &signature.to_signature().unwrap(), &public_key.0).is_ok()
}

// PublicKey.
// Deserialize.
impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let public_key = PublicKey::from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(PublicKey(public_key.0))
    }
}

// FromStr.
impl FromStr for PublicKey {
    type Err = secp256k1::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let public_key = secp256k1::PublicKey::from_str(s)?;
        Ok(PublicKey(public_key))
    }
}

// Serialize.
impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let public_key_str = self.0.to_string();
        serializer.serialize_str(&public_key_str)
    }
}

// Signature.
// Deserialize.
impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let b = hex::decode(s).map_err(serde::de::Error::custom)?;
        let signature =
            secp256k1::ecdsa::Signature::from_der(&b).map_err(serde::de::Error::custom)?;
        Ok(Signature(signature.serialize_der()))
    }
}

// FromStr.
impl FromStr for Signature {
    type Err = secp256k1::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let signature = secp256k1::ecdsa::Signature::from_der(s.as_bytes())?;
        Ok(Signature(signature.serialize_der()))
    }
}

// Serialize.
impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // get the signature bytes no length
        let b = self.0.as_ref();
        let mut signature_str = String::new();
        for i in b {
            signature_str.push_str(&format!("{:02x}", i));
        }
        serializer.serialize_str(&signature_str)
    }
}

// to_inner
impl Signature {
    pub fn to_inner(&self) -> secp256k1::ecdsa::SerializedSignature {
        self.0
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // use crypto::verify_signature;

    #[test]
    fn test_parse_keypair() {
        let keypair = ECDSAKeypair::new();
        let keypair2 = ECDSAKeypair::new_from_privatekey(&keypair.get_secret_key().display_secret().to_string(),);
        // Verify generated keypair.
        assert!(keypair2.get_secret_key().display_secret().to_string() == keypair.get_secret_key().display_secret().to_string());
        assert!(keypair2.get_public_key().to_string() == keypair.get_public_key().to_string());
    }
}
