use neotales_x509::ssh;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let certificate_path = std::env::args_os()
        .nth(1)
        .ok_or("usage: cargo run --example der_to_authorized_key -- <certificate.der>")?;
    let certificate_der = std::fs::read(certificate_path)?;
    let authorized_key = ssh::to_authorized_key(&certificate_der)?;
    println!("{authorized_key}");
    Ok(())
}
