pub struct Configuration {
    pub is_host: bool,
    pub client_port: u16,
    pub host_port: u16,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            is_host: false,
            client_port: 4791,
            host_port: 4792,
        }
    }
}