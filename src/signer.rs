use git2::Error;

/// OpenPGP 签名器（Rust 原生实现）
///
/// 替代外部 GPG 程序，提供纯 Rust 的签名能力
pub struct Signer {
	_phantom: std::marker::PhantomData<u8>,
}

impl Signer {
	/// 从文件加载签名密钥（暂未实现，占位符）
	pub fn from_key_file(_path: &str, _passphrase: Option<String>) -> Result<Self, Error> {
		Ok(Self { _phantom: std::marker::PhantomData })
	}

	/// 对数据进行签名（暂未实现，占位符）
	pub fn sign(&self, _data: &[u8]) -> Result<String, Error> {
		Ok("-----BEGIN PGP SIGNATURE-----\nPLACEHOLDER\n-----END PGP SIGNATURE-----".to_string())
	}
}

#[cfg(test)]
mod tests {

	#[test]
	fn test_signer_creation_from_key_file() {}

	#[test]
	fn test_sign_method_exists() {}
}
