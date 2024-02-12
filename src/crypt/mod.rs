mod error;
pub mod pwd;
pub mod token;

pub use error::{Error, Result};

use hmac::{Hmac, Mac};
use sha2::Sha512;

pub struct EncryptContent {
	pub content: String,
	pub salt: String,
}

pub fn encrypt_into_b64u(
	key: &[u8],
	enc_content: &EncryptContent,
) -> Result<String> {
	let EncryptContent { content, salt } = enc_content;

	// -- Create a HMAC-SHA-512 from key.
	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

	// -- Add content.
	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	// -- Finalize and b64u encode.
	let hmac_result = hmac_sha512.finalize();
	let result_bytes = hmac_result.into_bytes();

	let result = base64_url::encode(&result_bytes);

	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use rand::RngCore;

	#[test]
	fn test_encrypt_into_b64u_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mut fx_key = [0u8, 64];
		let fx_enc_content = EncryptContent {
			content: "hello world".to_string(),
			salt: "some pepper".to_string(),
		};

		// TODO: Need to fix fx_key, and precompute fx_res.
		let fx_res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

		// -- Exec
		let res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

		// -- Check
		assert_eq!(res, fx_res);

		Ok(())
	}
}
