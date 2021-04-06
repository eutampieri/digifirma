use xml;

const CA_URL: &'static str = "https://eidas.agid.gov.it/TL/TSL-IT.xml";

fn wrap_at(string: &str, size: usize) -> String {
    let mut v = vec![];
    let mut cur = string;
    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(std::cmp::min(size, cur.len()));
        v.push(chunk.clone());
        cur = rest;
    }
    v.join("\n")
}

pub fn get_ca() -> openssl::x509::store::X509Store {
    let client: Box<dyn super::http_client::HttpClient>;
    client = Box::new(super::http_client::Ureq {});
    let raw_xml = client.get(CA_URL).unwrap();
    let xml = xml::EventReader::from_str(&raw_xml);
    let mut keep: bool = false;
    xml.into_iter()
        .filter_map(|e| match e {
            Ok(xml::reader::XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "X509Certificate" {
                    keep = true;
                }
                None
            }
            Ok(xml::reader::XmlEvent::Characters(c)) => {
                if keep {
                    Some(c)
                } else {
                    None
                }
            }
            Ok(xml::reader::XmlEvent::EndElement { name }) => {
                if name.local_name == "X509Certificate" {
                    keep = false;
                }
                None
            }
            Err(_e) => None,
            _ => None,
        })
        .map(|x| wrap_at(&x, 64))
        .map(|x| {
            format!(
                "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----\n",
                x
            )
        })
        .map(|x| openssl::x509::X509::from_pem(x.as_bytes()).unwrap())
        .fold(
            openssl::x509::store::X509StoreBuilder::new().unwrap(),
            |mut s, i| {
                s.add_cert(i).unwrap();
                s
            },
        )
        .build()
}
