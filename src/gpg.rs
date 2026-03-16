use std::process::Command;

use git2::{Buf, Error, Repository};

/// GPG signing configuration from git config
pub struct GpgConfig {
	/// Whether GPG signing is enabled (commit.gpgSign)
	pub enabled: bool,
	/// GPG signing key (user.signingKey)
	pub signing_key: Option<String>,
	/// GPG program path (gpg.program)
	pub program: String,
}

impl GpgConfig {
	/// Load GPG configuration from repository config
	pub fn from_repo(repo: &Repository) -> Result<Self, Error> {
		let config = repo.config()?;

		let enabled = config.get_bool("commit.gpgSign").unwrap_or(false);

		let signing_key = config.get_string("user.signingKey").ok();

		let program = config
			.get_string("gpg.program")
			.unwrap_or_else(|_| "gpg".to_string());

		Ok(Self {
			enabled,
			signing_key,
			program,
		})
	}

	/// Check if GPG signing should be performed
	pub fn should_sign(&self) -> bool {
		self.enabled
	}

	/// Sign the given content using GPG
	///
	/// # Arguments
	/// * `content` - The content to sign (commit buffer)
	///
	/// # Returns
	/// The ASCII-armored signature
	pub fn sign(&self, content: &[u8]) -> Result<String, Error> {
		let mut cmd = Command::new(&self.program);

		// Use --status-fd for better error detection
		cmd.arg("--status-fd=2")
			.arg("--batch")
			.arg("--detach-sign")
			.arg("--armor");

		// Add signing key if specified
		if let Some(ref key) = self.signing_key {
			cmd.arg("--local-user").arg(key);
		}

		// Pass content via stdin
		cmd.stdin(std::process::Stdio::piped())
			.stdout(std::process::Stdio::piped())
			.stderr(std::process::Stdio::piped());

		let mut child = cmd
			.spawn()
			.map_err(|e| Error::from_str(&format!("Failed to spawn GPG process: {}", e)))?;

		// Write content to stdin
		use std::io::Write;
		if let Some(mut stdin) = child.stdin.take() {
			stdin
				.write_all(content)
				.map_err(|e| Error::from_str(&format!("Failed to write to GPG stdin: {}", e)))?;
		}

		let output = child
			.wait_with_output()
			.map_err(|e| Error::from_str(&format!("Failed to wait for GPG process: {}", e)))?;

		if !output.status.success() {
			let stderr = String::from_utf8_lossy(&output.stderr);
			return Err(Error::from_str(&format!("GPG signing failed: {}", stderr)));
		}

		let signature = String::from_utf8(output.stdout)
			.map_err(|e| Error::from_str(&format!("Invalid UTF-8 in GPG signature: {}", e)))?;

		Ok(signature)
	}
}

/// Sign a commit buffer and write to repository
///
/// # Arguments
/// * `repo` - The git repository
/// * `commit_buf` - The unsigned commit buffer from commit_create_buffer
/// * `signature` - The GPG signature
///
/// # Returns
/// The OID of the signed commit
pub fn commit_with_signature(
	repo: &Repository,
	commit_buf: &Buf,
	signature: &str,
) -> Result<git2::Oid, Error> {
	// Convert Buf to str for commit_signed
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

		// Default should be disabled
		assert!(!config.enabled);
		assert!(config.signing_key.is_none());
		assert_eq!(config.program, "gpg");
	}
}
