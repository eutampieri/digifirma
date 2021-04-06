mod ca;
mod http_client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let ca = super::ca::get_ca();
        assert_eq!(2 + 2, 4);
        let file = std::fs::read("prova.p7s").unwrap();
        let empty_stack = openssl::stack::Stack::new().unwrap();
        let cades = openssl::pkcs7::Pkcs7::from_der(&file).unwrap();
        let mut out = vec![];
        let ver = cades.verify(
            &empty_stack.as_ref(),
            &ca,
            None,
            Some(&mut out),
            openssl::pkcs7::Pkcs7Flags::NOVERIFY, //TODO FIX
        );
        let crts = cades
            .signers(&&empty_stack.as_ref(), openssl::pkcs7::Pkcs7Flags::NOVERIFY)
            .unwrap()
            .iter()
            .map(|x| x.to_owned())
            .collect::<Vec<openssl::x509::X509>>();
        println!("{:?}", crts);
    }
}
