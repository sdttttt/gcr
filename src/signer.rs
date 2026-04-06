use git2::Error;
use pgp::{
	composed::{Deserializable, SignedSecretKey, StandaloneSignature},
	crypto::hash::HashAlgorithm,
	packet::{SignatureConfig, SignatureType, SignatureVersion},
	types::KeyTrait,
};
use std::fs::File;
use std::io::Cursor;

/// OpenPGP 签名器（Rust 原生实现）
///
/// 使用 rPGP 库实现纯 Rust 的 PGP detached 签名
#[derive(Debug)]
pub struct Signer {
	secret_key: SignedSecretKey,
	passphrase: Option<String>,
}

impl Signer {
	/// 从文件加载签名密钥
	pub fn from_key_file(path: &str, passphrase: Option<String>) -> Result<Self, Error> {
		let file = File::open(path)
			.map_err(|e| Error::from_str(&format!("Failed to open key file '{}': {}", path, e)))?;

		// 从 ASCII-armored 文件解析 SignedSecretKey
		let (secret_key, _headers) = SignedSecretKey::from_armor_single(file)
			.map_err(|e| Error::from_str(&format!("Failed to parse PGP key: {:?}", e)))?;

		// 验证密钥自签名
		secret_key
			.verify()
			.map_err(|e| Error::from_str(&format!("PGP key verification failed: {:?}", e)))?;

		Ok(Self { secret_key, passphrase })
	}

	/// 对数据进行 detached 签名，返回 ASCII-armored 签名
	pub fn sign(&self, data: &[u8]) -> Result<String, Error> {
		let signing_key = &self.secret_key.primary_key;
		let passphrase = self.passphrase.as_deref().unwrap_or("");

		// 创建签名配置（不需要 subpackets）
		let sig_config = SignatureConfig::new_v4(
			SignatureVersion::V4,
			SignatureType::Binary,
			signing_key.algorithm(),
			HashAlgorithm::SHA2_256,
			vec![],
			vec![],
		);

		// 对数据进行签名
		let signature = sig_config
			.sign(signing_key, || passphrase.to_string(), Cursor::new(data))
			.map_err(|e| Error::from_str(&format!("PGP signing failed: {:?}", e)))?;

		// 转换为 StandaloneSignature 并输出 ASCII-armored 格式
		let standalone_sig = StandaloneSignature::new(signature);
		standalone_sig
			.to_armored_string(Default::default())
			.map_err(|e| Error::from_str(&format!("Failed to armor signature: {:?}", e)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_key_file_nonexistent() {
		let result = Signer::from_key_file("/nonexistent/path.asc", None);
		assert!(result.is_err());
		let err_msg = result.unwrap_err().message().to_string();
		assert!(err_msg.contains("Failed to open key file"));
	}

	#[test]
	fn test_from_key_file_invalid_content() {
		use tempfile::NamedTempFile;
		use std::io::Write;

		let mut tmp = NamedTempFile::new().unwrap();
		tmp.write_all(b"this is not a pgp key").unwrap();
		tmp.flush().unwrap();

		let result = Signer::from_key_file(tmp.path().to_str().unwrap(), None);
		assert!(result.is_err());
	}

	#[test]
	fn test_sign_with_real_key() {
		let key_path = "tests/fixtures/test_secret_key.asc";
		if !std::path::Path::new(key_path).exists() {
			return;
		}

		let signer = Signer::from_key_file(key_path, None).expect("failed to load key");
		let data = b"test commit message";
		let sig = signer.sign(data).expect("failed to sign");

		// 验证签名格式
		assert!(sig.starts_with("-----BEGIN PGP SIGNATURE-----"));
		assert!(sig.contains("-----END PGP SIGNATURE-----"));
	}
}

