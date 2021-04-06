/*!
Easily verify Italian CIE signed files.
*/
mod ca;
mod http_client;
mod nid_mapping;
mod verify;

use codice_fiscale::CodiceFiscale;
pub use verify::verify;

/// A person, i.e. the entity that signs a file
#[derive(Debug)]
pub struct Person {
    /// Name
    pub name: String,
    /// Surname
    pub surname: String,
    /// Italian fiscal code, useful to get date and place of birth
    pub fiscal_code: CodiceFiscale,
    /// The serial number of the ID card
    pub document_id: String,
}

/// The result of the verification
#[derive(Debug)]
pub struct SignedMessage {
    /// The file which has been signed.
    /// This is the unsigned file.
    pub file: Vec<u8>,
    /// A list of people who signed this file.
    pub signers: Vec<Person>,
}

#[cfg(test)]
mod tests {
    /*#[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let file = std::fs::read("prova.p7s").unwrap();
        println!("{:?}", super::verify(&file).unwrap().signers);
    }*/
}
