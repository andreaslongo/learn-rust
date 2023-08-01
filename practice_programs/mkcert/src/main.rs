use openssl::asn1::Asn1Integer;
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::bn::MsbOption;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use openssl::x509::extension::AuthorityKeyIdentifier;
use openssl::x509::extension::BasicConstraints;
use openssl::x509::extension::SubjectKeyIdentifier;
use openssl::x509::X509NameBuilder;
use openssl::x509::X509;

fn main() {
    new_self_signed_certificate()
}

fn new_self_signed_certificate() {
    // Creates a new self-signed certificate.
    let key_pair = new_key_pair();

    let mut x509_name = X509NameBuilder::new().unwrap();
    x509_name.append_entry_by_text("CN", "example.com").unwrap();
    let x509_name = x509_name.build();

    let mut cert_builder = X509::builder().unwrap();

    cert_builder.set_version(2).unwrap();
    let serial_number = new_serial_number();

    cert_builder.set_serial_number(&serial_number).unwrap();
    cert_builder.set_issuer_name(&x509_name).unwrap();
    let not_before = Asn1Time::days_from_now(0).unwrap();
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = Asn1Time::days_from_now(365).unwrap();
    cert_builder.set_not_after(&not_after).unwrap();
    cert_builder.set_subject_name(&x509_name).unwrap();
    cert_builder.set_pubkey(&key_pair).unwrap();

    let subject_key_identifier = SubjectKeyIdentifier::new()
        .build(&cert_builder.x509v3_context(None, None))
        .unwrap();
    cert_builder
        .append_extension(subject_key_identifier)
        .unwrap();

    let authority_key_identifier = AuthorityKeyIdentifier::new()
        .keyid(true)
        .build(&cert_builder.x509v3_context(None, None))
        .unwrap();
    cert_builder
        .append_extension(authority_key_identifier)
        .unwrap();

    cert_builder
        .append_extension(BasicConstraints::new().critical().ca().build().unwrap())
        .unwrap();

    cert_builder
        .sign(&key_pair, MessageDigest::sha256())
        .unwrap();
    let cert = cert_builder.build();

    // Prints the public key.
    let pub_key: Vec<u8> = key_pair.public_key_to_pem().unwrap();
    print!("{}", std::str::from_utf8(pub_key.as_slice()).unwrap());

    // Prints the certificate.
    let c: Vec<u8> = cert.to_text().unwrap();
    print!("{}", std::str::from_utf8(c.as_slice()).unwrap());
}

fn new_key_pair() -> PKey<Private> {
    // Generates a new RSA 2048-bit public/private key pair.
    let rsa = Rsa::generate(2048).unwrap();
    PKey::from_rsa(rsa).unwrap()
}

fn new_serial_number() -> Asn1Integer {
    // Generates a new certificate serial number.
    let mut serial = BigNum::new().unwrap();
    serial.rand(159, MsbOption::MAYBE_ZERO, false).unwrap();
    serial.to_asn1_integer().unwrap()
}
