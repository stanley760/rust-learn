//! Secp256k1 椭圆曲线密码学实现
//! 使用 k256 crate 实现 ECDSA 签名、验证和密钥管理

use k256::ecdsa::{
    signature::Signer as K256Signer,
    signature::Verifier as K256Verifier,
    RecoveryId, Signature, SigningKey, VerifyingKey,
};
use k256::sha2::{Digest, Sha256};
use rand::rngs::OsRng;
use std::fs;

/// Secp256k1 签名器
#[derive(Debug, Clone)]
pub struct Secp256k1Signer {
    signing_key: SigningKey,
}

/// Secp256k1 验证器
#[derive(Debug, Clone)]
pub struct Secp256k1Verifier {
    verifying_key: VerifyingKey,
}

/// 密钥对
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}
#[allow(unused)]
impl Secp256k1Signer {
    /// 从私钥字节创建签名器
    pub fn from_bytes(private_key: &[u8]) -> Result<Self, Secp256k1Error> {
        let signing_key = SigningKey::from_slice(private_key)
            .map_err(|_| Secp256k1Error::InvalidPrivateKey)?;
        Ok(Self { signing_key })
    }

    /// 生成新的随机密钥对
    pub fn generate() -> Self {
        Self {
            signing_key: SigningKey::random(&mut OsRng),
        }
    }

    /// 对数据进行签名（先计算 SHA256 哈希）
    pub fn sign(&self, data: &[u8]) -> Signature {
        let hash = Sha256::digest(data);
        self.signing_key.sign(&hash)
    }

    /// 对预计算的哈希值进行签名
    pub fn sign_hash(&self, hash: [u8; 32]) -> Signature {
        self.signing_key.sign(&hash)
    }

    /// 创建带恢复 ID 的签名（用于公钥恢复）
    pub fn sign_recoverable(&self, data: &[u8]) -> (Signature, RecoveryId) {
        self.signing_key
            .sign_recoverable(data)
            .expect("sign_recoverable failed")
    }

    /// 获取私钥字节
    pub fn to_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes().into()
    }

    /// 获取公钥
    pub fn verifying_key(&self) -> VerifyingKey {
        *self.signing_key.verifying_key()
    }

    /// 获取压缩格式的公钥
    pub fn public_key_compressed(&self) -> Vec<u8> {
        self.signing_key.verifying_key().to_sec1_bytes().to_vec()
    }

    /// 获取未压缩格式的公钥
    pub fn public_key_uncompressed(&self) -> Vec<u8> {
        let point = self.signing_key.verifying_key().to_encoded_point(false);
        point.as_bytes().to_vec()
    }

    /// 导出密钥对
    pub fn export_keypair(&self) -> KeyPair {
        KeyPair {
            private_key: self.to_bytes().to_vec(),
            public_key: self.public_key_compressed(),
        }
    }

    /// 从文件加载私钥
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> Result<Self, Secp256k1Error> {
        let key_data = fs::read(path)?;
        Self::from_bytes(&key_data)
    }

    /// 保存私钥到文件
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), Secp256k1Error> {
        fs::write(path, self.to_bytes())?;
        Ok(())
    }

    /// 从助记词生成密钥（简化版，实际应用建议使用 bip39 库）
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_slice(seed).unwrap();
        Self { signing_key }
    }
}
#[allow(unused)]
impl Secp256k1Verifier {
    /// 从公钥字节创建验证器（支持压缩和未压缩格式）
    pub fn from_bytes(public_key: &[u8]) -> Result<Self, Secp256k1Error> {
        let verifying_key = VerifyingKey::from_sec1_bytes(public_key)
            .map_err(|_| Secp256k1Error::InvalidPublicKey)?;
        Ok(Self { verifying_key })
    }

    /// 从签名器创建验证器
    pub fn from_signer(signer: &Secp256k1Signer) -> Self {
        Self {
            verifying_key: signer.verifying_key(),
        }
    }

    /// 验证签名（先计算 SHA256 哈希）
    pub fn verify(&self, data: &[u8], signature: &Signature) -> Result<bool, Secp256k1Error> {
        let hash = Sha256::digest(data);
        Ok(self.verifying_key.verify(hash.as_ref(), signature).is_ok())
    }

    /// 验证预计算哈希的签名
    pub fn verify_hash(&self, hash: [u8; 32], signature: &Signature) -> Result<bool, Secp256k1Error> {
        Ok(self.verifying_key.verify(&hash, signature).is_ok())
    }

    /// 使用恢复 ID 从签名恢复公钥
    pub fn recover_from_signature(
        data: &[u8],
        signature: &Signature,
        recovery_id: RecoveryId,
    ) -> Result<VerifyingKey, Secp256k1Error> {
        let hash = Sha256::digest(data);
        VerifyingKey::recover_from_prehash(hash.as_ref(), signature, recovery_id)
            .map_err(|_| Secp256k1Error::RecoveryFailed)
    }

    /// 获取公钥字节
    pub fn to_bytes(&self) -> Vec<u8> {
        self.verifying_key.to_sec1_bytes().to_vec()
    }

    /// 从文件加载公钥
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> Result<Self, Secp256k1Error> {
        let key_data = fs::read(path)?;
        Self::from_bytes(&key_data)
    }

    /// 保存公钥到文件
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), Secp256k1Error> {
        fs::write(path, self.to_bytes())?;
        Ok(())
    }
}
#[allow(unused)]
/// Secp256k1 错误类型
#[derive(Debug, thiserror::Error)]
pub enum Secp256k1Error {
    #[error("无效的私钥")]
    InvalidPrivateKey,

    #[error("无效的公钥")]
    InvalidPublicKey,

    #[error("签名验证失败")]
    VerificationFailed,

    #[error("公钥恢复失败")]
    RecoveryFailed,

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("密钥长度错误")]
    InvalidKeyLength,
}

impl From<k256::ecdsa::Error> for Secp256k1Error {
    fn from(_: k256::ecdsa::Error) -> Self {
        Secp256k1Error::VerificationFailed
    }
}
#[allow(unused)]
/// 密钥衍生函数（简化版 HD 钱包功能）
pub struct KeyDerivation;
#[allow(unused)]
impl KeyDerivation {
    /// 从父私钥派生子私钥（简化实现）
    /// 注意：实际应用应使用 BIP-32 标准
    pub fn derive_child(
        parent_key: &SigningKey,
        index: u32,
    ) -> Result<SigningKey, Secp256k1Error> {
        // 简化实现：使用 HMAC-SHA256 派生
        use hmac::Mac;
        use hmac::Hmac;

        let parent_bytes = parent_key.to_bytes();
        let chain_code = [0u8; 32]; // 实际应用中应有真实的链码

        let mut hmac = Hmac::<Sha256>::new_from_slice(&chain_code).unwrap();
        hmac.update(&parent_bytes);
        hmac.update(&index.to_be_bytes());
        let result = hmac.finalize();
        let bytes = result.into_bytes();

        let child_key = SigningKey::from_slice(&bytes[..32])
            .map_err(|_| Secp256k1Error::InvalidKeyLength)?;
        Ok(child_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let signer = Secp256k1Signer::generate();
        let private_key = signer.to_bytes();
        assert_eq!(private_key.len(), 32);

        let public_key = signer.public_key_compressed();
        assert_eq!(public_key.len(), 33); // 压缩格式公钥
    }

    #[test]
    fn test_sign_and_verify() {
        let signer = Secp256k1Signer::generate();
        let verifier = Secp256k1Verifier::from_signer(&signer);

        let data = b"Hello, Secp256k1!";
        let signature = signer.sign(data);

        let is_valid = verifier.verify(data, &signature).unwrap();
        assert!(is_valid);

        // 测试错误数据
        let wrong_data = b"Wrong data";
        let is_valid = verifier.verify(wrong_data, &signature).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_key_serialization() {
        let signer = Secp256k1Signer::generate();
        let private_bytes = signer.to_bytes();
        let public_bytes = signer.public_key_compressed();

        // 从字节重新创建
        let signer2 = Secp256k1Signer::from_bytes(&private_bytes).unwrap();
        let verifier = Secp256k1Verifier::from_bytes(&public_bytes).unwrap();

        let data = b"Test data";
        let signature = signer2.sign(data);
        assert!(verifier.verify(data, &signature).unwrap());
    }

    #[test]
    fn test_recoverable_signature() {
        let signer = Secp256k1Signer::generate();
        let original_key = signer.verifying_key();

        let data = b"Recoverable signature test";
        let (signature, recovery_id) = signer.sign_recoverable(data);

        let recovered_key =
            Secp256k1Verifier::recover_from_signature(data, &signature, recovery_id).unwrap();

        assert_eq!(original_key, recovered_key);
    }

    #[test]
    fn test_keypair_export() {
        let signer = Secp256k1Signer::generate();
        let keypair = signer.export_keypair();

        assert_eq!(keypair.private_key.len(), 32);
        assert_eq!(keypair.public_key.len(), 33);

        // 可以从导出的密钥重建
        let new_signer = Secp256k1Signer::from_bytes(&keypair.private_key).unwrap();
        let verifier = Secp256k1Verifier::from_bytes(&keypair.public_key).unwrap();

        let data = b"KeyPair export test";
        let signature = new_signer.sign(data);
        assert!(verifier.verify(data, &signature).unwrap());
    }
}
