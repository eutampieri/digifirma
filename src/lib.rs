mod ca;
mod http_client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::ca::get_ca();
        assert_eq!(2 + 2, 4);
    }
}
