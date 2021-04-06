use lazy_static::lazy_static;
use openssl::error::ErrorStack;

use super::nid_mapping;
use super::nid_mapping::Nid;
use crate::SignedMessage;

lazy_static! {
    static ref AUTHORITIES: openssl::x509::store::X509Store = super::ca::get_ca();
    static ref EMPTY_STACK: openssl::stack::Stack<openssl::x509::X509> =
        openssl::stack::Stack::new().unwrap();
}

pub fn verify(der: &[u8]) -> Result<SignedMessage, ErrorStack> {
    let cades = openssl::pkcs7::Pkcs7::from_der(der)?;
    let mut file = vec![];
    cades.verify(
        &EMPTY_STACK.as_ref(),
        &AUTHORITIES,
        None,
        Some(&mut file),
        openssl::pkcs7::Pkcs7Flags::NOVERIFY, //TODO FIX
    )?;
    let signers = cades
        .signers(
            &EMPTY_STACK.as_ref(),
            openssl::pkcs7::Pkcs7Flags::from_bits_truncate(0),
        )?
        .iter()
        .map(|x| x.to_owned())
        .map(|x| {
            let subject_name = x.subject_name();
            super::Person {
                name: nid_mapping::get(Nid::GivenName, subject_name)[0].clone(),
                surname: nid_mapping::get(Nid::Surname, subject_name)[0].clone(),
                fiscal_code: codice_fiscale::CodiceFiscale::parse(
                    nid_mapping::get(Nid::CommonName, subject_name)[0]
                        .split("/")
                        .nth(0)
                        .unwrap(),
                )
                .unwrap(),
                document_id: nid_mapping::get(Nid::SerialNumber, subject_name)[0]
                    .split("-")
                    .nth(1)
                    .unwrap()
                    .to_owned(),
            }
        })
        .collect();
    Ok(SignedMessage { file, signers })
}
