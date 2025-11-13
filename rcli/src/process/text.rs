use crate::{read_file, TextSignFormat};
use anyhow::{Ok, Result};
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::{fs, io::Read};

pub trait TextSign {
    /// sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    /// verify the signature of the data from the reader
    fn verify(&self, data: impl Read, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<std::path::Path>) -> Result<Self>
    where
        Self: Sized;
}

pub struct Blake3 {
    pub key: [u8; 32],
}

pub struct Ed25519Signer {
    pub key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut data: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        data.read_to_end(&mut buf)?;
        // notes: as_bytes() don't provide constant-time equality checking
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes() == sig)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(self.key.sign(&buf).to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut data: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        data.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = read_file(input)?;

    let result = match format {
        TextSignFormat::Blake3 => {
            let sign = Blake3::load(key)?;
            sign.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let load = Ed25519Signer::load(key)?;
            load.sign(&mut reader)?
        }
    };
    let result = BASE64_URL_SAFE_NO_PAD.encode(result);
    println!("{}", result);
    Ok(())
}

pub fn process_verify(input: &str, key: &str, format: TextSignFormat, sig: &str) -> Result<()> {
    let mut reader = read_file(input)?;
    let sig = BASE64_URL_SAFE_NO_PAD.decode(sig)?;

    let result = match format {
        TextSignFormat::Blake3 => {
            let sign = Blake3::load(key)?;
            sign.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let load = Ed25519Verifier::load(key)?;
            load.verify(&mut reader, &sig)?
        }
    };
    println!("{}", result);
    Ok(())
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into().expect("invalid key");
        Ok(Blake3::new(key))
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Ed25519Signer::new(key))
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Ed25519Verifier::new(key))
    }
}
