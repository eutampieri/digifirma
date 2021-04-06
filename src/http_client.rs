#[derive(Debug)]
pub enum HttpClientError {
    //IoError(std::io::Error),
    HttpError(httpstatus::StatusCode),
}

pub trait HttpClient {
    fn get(url: &str) -> Result<String, HttpClientError>;
}

pub struct Ureq {}
impl HttpClient for Ureq {
    fn get(url: &str) -> Result<String, HttpClientError> {
        let response = ureq::get(url).call().unwrap();
        let status = httpstatus::StatusCode::from(response.status());
        if status.class() == httpstatus::StatusClass::Success {
            Ok(response.into_string().unwrap())
        } else {
            Err(HttpClientError::HttpError(status))
        }
    }
}
