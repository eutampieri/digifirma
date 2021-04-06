pub enum Nid {
    CommonName,
    SerialNumber,
    GivenName,
    Surname,
}

impl From<Nid> for openssl::nid::Nid {
    fn from(nid: Nid) -> Self {
        let raw = match nid {
            Nid::CommonName => 13,
            Nid::SerialNumber => 105,
            Nid::GivenName => 99,
            Nid::Surname => 100,
        };
        Self::from_raw(raw)
    }
}

pub fn get(nid: Nid, data: &openssl::x509::X509NameRef) -> Vec<String> {
    data.entries_by_nid(nid.into())
        .map(|x| x.data().as_utf8().unwrap())
        .map(|x| x.chars().collect())
        .collect::<Vec<String>>()
}
