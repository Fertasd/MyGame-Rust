pub struct ServerConfig {
    port: u16
}

impl ServerConfig {
    fn new(port: u16) -> ServerConfig {
        ServerConfig { port: port }
    }

    fn listen(self) {

    }
}