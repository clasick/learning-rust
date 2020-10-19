pub mod network;

mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::client;

        client::connect();
    }
}
