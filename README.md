# neotales-x509

Convert X.509 certificates and OpenSSH keys between DER, PEM, and SSH formats.

![Neotales](https://github.com/neotales/brand/raw/main/assets/logo.png)

Need to move a certificate into an SSH public-key field, or turn an OpenSSH key pair into an X.509 identity? This crate handles the encoding work.

![A key becomes a stamped certificate](docs/media/stamp.gif)

**Certificate stamp, applied.** You choose the trust policy; this crate parses and generates the certificate data.

## Install

```sh
cargo add neotales-x509
```

## What you can do

- **Convert DER certificates to RFC 6187 SSH blobs or authorized-key lines.** Use `ssh::to_ssh_wire` and `ssh::to_authorized_key`. Reverse the conversion with `ssh::from_ssh_wire` and `ssh::from_authorized_key`.
- **Build an X.509 key pair from an OpenSSH key pair.** `ssh_keypair_to_x509` returns a self-signed certificate, PKCS#8 private-key PEM, and SPKI public-key PEM. It accepts Ed25519, P-256 ECDSA, and RSA. OpenSSH private keys may use bcrypt and AES-256-CTR encryption.
- **Convert OpenSSH certificates and X.509 identities.** Use `openssh_cert::x509_to_openssh_certificate`, `openssh_cert::parse`, `openssh_cert::to_wire`, and `openssh_cert::openssh_certificate_to_x509_identity`.
- **Parse and verify SSH certificates.** `ssh_cert::parse_certificate` parses the certificate, and `ssh_cert::verify_certificate` checks its signature.
- **Encode and decode PEM.** `pem::encode` and `pem::decode` handle one DER block.
- **Use named object identifiers.** The crate exports `ED25519_OID` and the `oids` module.

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

## Test

```sh
cargo test
```

## Licence

MIT. See [LICENSE.md](LICENSE.md).
