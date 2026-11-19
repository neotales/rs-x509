# neotales-x509

Turn an OpenSSH key pair into a self-signed X.509 certificate, or carry an X.509 certificate as an RFC 6187 authorized-key line. Convert OpenSSH certificate wire data, encode and decode PEM, and use named OIDs instead of scattering identifier strings through your code. You choose who to trust; this crate handles the bytes.

![Neotales](https://github.com/neotales/brand/raw/main/assets/logo.png)

![A key becomes a stamped certificate](docs/media/stamp.gif)

![A key becomes an X.509 certificate](docs/media/meme.png)

**OpenSSH key in, X.509 certificate out. The format conversion is the easy part.**

## Install 📦

```sh
cargo add neotales-x509
```

## What you can do 🔧

- 🔁 **Convert DER certificates to RFC 6187 SSH blobs or authorized-key lines.** Use `ssh::to_ssh_wire` and `ssh::to_authorized_key`. Reverse the conversion with `ssh::from_ssh_wire` and `ssh::from_authorized_key`.
- 🔑 **Build an X.509 key pair from an OpenSSH key pair.** `ssh_keypair_to_x509` returns a self-signed certificate, PKCS#8 private-key PEM, and SPKI public-key PEM. It accepts Ed25519, P-256 ECDSA, and RSA. OpenSSH private keys may use bcrypt and AES-256-CTR encryption.
- 🔄 **Convert OpenSSH certificates and X.509 identities.** Use `openssh_cert::x509_to_openssh_certificate`, `openssh_cert::parse`, `openssh_cert::to_wire`, and `openssh_cert::openssh_certificate_to_x509_identity`.
- ✅ **Parse and verify SSH certificates.** `ssh_cert::parse_certificate` parses the certificate, and `ssh_cert::verify_certificate` checks its signature.
- 📄 **Encode and decode PEM.** `pem::encode` and `pem::decode` handle one DER block.
- 🏷️ **Use named object identifiers.** The crate exports `ED25519_OID` and the `oids` module.

The SSH certificate conversions support RSA, P-256 ECDSA, and Ed25519. An OpenSSH certificate contains no private key.

## Convert a DER certificate

This runnable example takes DER bytes and prints the corresponding authorized-key line:

```rust
use neotales_x509::ssh;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let certificate_der = std::fs::read("certificate.der")?;
    let authorized_key = ssh::to_authorized_key(&certificate_der)?;
    println!("{authorized_key}");
    Ok(())
}
```

Run it with `cargo run --example der_to_authorized_key -- certificate.der` after saving the code as `examples/der_to_authorized_key.rs`.

## Create a self-signed X.509 key pair 🔐

This is the call shape for converting an OpenSSH private key and matching public key. The fixture directory has no OpenSSH private key, so this sample is not a runnable example and has not been executed. Supply key contents from your own source.

```rust
use neotales_x509::{SshKeyToX509, ssh_keypair_to_x509};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let private_key: &str = "contents of an OpenSSH private key";
    let public_key: &str = "ssh-ed25519 ...";

    let key_pair = ssh_keypair_to_x509(
        private_key,
        public_key,
        &SshKeyToX509 {
            common_name: "host.example".to_owned(),
            not_before_unix: 1_700_000_000,
            not_after_unix: 1_731_536_000,
        },
        None,
    )?;

    let certificate_pem: String = key_pair.certificate_pem;
    let private_key_pem = key_pair.private_key_pem;
    let public_key_pem: String = key_pair.public_key_pem;
    println!("{certificate_pem}");
    println!("{private_key_pem}");
    println!("{public_key_pem}");
    Ok(())
}
```

The fields shown are `certificate_pem: String`, `private_key_pem: Zeroizing<String>`, and `public_key_pem: String`. This is a call-shape example, not a runnable key fixture. No OpenSSH private key exists in `tests/fixtures`, so the sample has not been executed.

## Test 🧪

```sh
cargo test
```

## Licence 📜

MIT. See [LICENSE.md](LICENSE.md).
