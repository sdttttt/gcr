use git2::{Buf, Error, Repository};

use crate::signer::Signer;

/// PGP signing configuration from git config
pub struct GpgConfig {
	/// Whether PGP signing is enabled (commit.gpgSign)
	pub enabled: bool,
	/// Path to signing key file (gpg.keyPath)
	pub key_path: Option<String>,
	/// Passphrase for signing key (gpg.passphraseEnv) - environment variable name
	pub passphrase_env: Option<String>,
}

impl GpgConfig {
	/// Load PGP signing configuration from repository config
	pub fn from_repo(repo: &Repository) -> Result<Self, Error> {
		let config = repo.config()?;
		let enabled = config.get_bool("commit.gpgSign").unwrap_or(false);
		let key_path = config.get_string("gpg.keyPath").ok();
		let passphrase_env = config.get_string("gpg.passphraseEnv").ok();

		Ok(Self { enabled, key_path, passphrase_env })
	}

	/// Check if PGP signing should be performed
	pub fn should_sign(&self) -> bool {
		self.enabled
	}

	/// Create native Rust PGP signer if key path is configured
	pub fn create_signer(&self) -> Result<Signer, Error> {
		let key_path = self
			.key_path
			.as_ref()
			.ok_or_else(|| Error::from_str("No key path configured for PGP signer"))?;

		let passphrase = self
			.passphrase_env
			.as_ref()
			.and_then(|env_name| std::env::var(env_name).ok());

		Signer::from_key_file(key_path, passphrase)
	}
}

/// Write a signed commit to repository
pub fn commit_with_signature(
	repo: &Repository,
	commit_buf: &Buf,
	signature: &str,
) -> Result<git2::Oid, Error> {
	let commit_content = std::str::from_utf8(commit_buf)
		.map_err(|e| Error::from_str(&format!("Invalid UTF-8 in commit buffer: {}", e)))?;
	repo.commit_signed(commit_content, signature, None)
}

#[cfg(test)]
mod tests {
	use super::*;
	use git2::RepositoryInitOptions;
	use tempfile::TempDir;

	#[test]
	fn test_gpg_config_defaults() {
		let tmp = TempDir::new().unwrap();
		let repo = Repository::init_opts(tmp.path(), &RepositoryInitOptions::new()).unwrap();

		let config = GpgConfig::from_repo(&repo).unwrap();

		assert!(!config.enabled);
		assert!(config.key_path.is_none());
		assert!(!config.should_sign());
	}

	#[test]
	fn test_gpg_config_enabled() {
		let tmp = TempDir::new().unwrap();
		let repo = Repository::init_opts(tmp.path(), &RepositoryInitOptions::new()).unwrap();

		{
			let mut cfg = repo.config().unwrap();
			cfg.set_bool("commit.gpgSign", true).unwrap();
			cfg.set_str("gpg.keyPath", "/path/to/key.asc").unwrap();
		}

		let config = GpgConfig::from_repo(&repo).unwrap();

		assert!(config.enabled);
		assert!(config.should_sign());
		assert_eq!(config.key_path.as_deref(), Some("/path/to/key.asc"));
	}

	#[test]
	fn test_create_signer_fails_without_key_path() {
		let config = GpgConfig { enabled: true, key_path: None, passphrase_env: None };
		let result = config.create_signer();
		assert!(result.is_err());
		assert!(result.unwrap_err().message().contains("No key path"));
	}

	#[test]
	fn test_first_commit_head_is_symbolic() {
		let tmp = TempDir::new().unwrap();
		let repo = Repository::init_opts(tmp.path(), &RepositoryInitOptions::new()).unwrap();

		let head = repo.find_reference("HEAD").unwrap();
		assert!(head.symbolic_target().is_some(), "HEAD should be a symbolic ref");
	}
}
