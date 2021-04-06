mod ca;
mod http_client;
mod nid_mapping;
mod verify;

use codice_fiscale::CodiceFiscale;
pub use verify::verify;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub fiscal_code: CodiceFiscale,
    pub document_id: String,
}

#[derive(Debug)]
pub struct SignedMessage {
    pub file: Vec<u8>,
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
