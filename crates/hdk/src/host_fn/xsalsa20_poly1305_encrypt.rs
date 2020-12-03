use crate::prelude::*;

/// Libsodium secret-key authenticated encryption: secretbox.
///
/// Libsodium symmetric encryption (a shared key to encrypt/decrypt) is called secretbox.
/// Secretbox can be used directly to hide data and is part of cryptographic systems such as
/// [saltpack](https://saltpack.org/).
///
/// Important information about secretbox:
///  - Wasm memory is NOT secure, a compromised host can steal the key.
///  - The key is SECRET, anyone with the key and nonce can read the encrypted message.
///  - The nonce is PUBLIC and UNIQUE, it must NEVER be re-used.
///  - It is STRONGLY RECOMMENDED to use `TryFromRandom` for the key and nonce for every message.
///  - Secretbox is designed for 'small' data, break large data into chunks with unique nonces.
///  - Secretbox is NOT quantum resistant.
///
/// If you want to hide data:
///  - Consider using capability tokens and/or dedicated DHT networks to control access.
///  - Consider how the shared key is being distributed, e.g. maybe use a key exchange protocol.
///  - Consider that a hybrid approach between network access + encryption might be best.
///  - Consider that encrypted data cannot be validated effectively by the public DHT.
///
/// The main use-case is to control access to data that may be broadcast across a semi-trusted or
/// untrusted context, where the intended recipients have all negotiated or shared a key outside
/// that context.
///
/// If you want to encrypt content so that a _specific_ recipient (i.e. public key) can decrypt it
/// then see the libsodium `box` algorithm or similar.
///
/// @see https://doc.libsodium.org/secret-key_cryptography/secretbox
/// @see https://nacl.cr.yp.to/secretbox.html
pub fn xsalsa20_poly1305_encrypt(
    key: XSalsa20Poly1305Key,
    nonce: XSalsa20Poly1305Nonce,
    data: XSalsa20Poly1305Data,
) -> HdkResult<XSalsa20Poly1305EncryptedData> {
    host_externs!(__xsalsa20_poly1305_encrypt);
    Ok(
        host_call::<XSalsa20Poly1305EncryptInput, XSalsa20Poly1305EncryptOutput>(
            __xsalsa20_poly1305_encrypt,
            &XSalsa20Poly1305EncryptInput::new((key, nonce, data)),
        )?
        .into_inner(),
    )
}

// Alias to secretbox as algorithm name.
pub fn secretbox(
    key: SecretBoxKey,
    nonce: SecretBoxNonce,
    data: SecretBoxData,
) -> HdkResult<SecretBoxEncryptedData> {
    xsalsa20_poly1305_encrypt(key, nonce, data)
}
