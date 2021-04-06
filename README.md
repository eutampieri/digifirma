# digifirma

digifirma aims to provide an easy to use interface to verify digitally signed documents exported from the [CieSign](https://forum.italia.it/t/ciesign-firma-con-cie/18273) app.

At the moment, there's a missing feature in Rust OpenSSL bindings that prevents this crate from verifying the authenticity of the certificate.

## Usage

```rust
let file = std::fs::read("file.p7m").unwrap();
let data = digifirma::verify(&file).unwrap();
std::fs::write("output", data.file); // Write the content which has been signed to a file
let signer = data.signers[0];
assert_eq!("MARIO", signer.name);
assert_eq!("ROSSI", signer.surname);
assert_eq!("AB12345CD", signer.document_id);
assert_eq!("RSSMRO50E01D634H", signer.fiscal_code.get_codice());
```
