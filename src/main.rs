use simple_proxy::middlewares::{router::*, Logger};
use simple_proxy::{Environment, SimpleProxy};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    port: u16,
}

#[derive(Debug, Clone)]
pub struct Config();

impl RouterConfig for Config {
    fn get_router_filename(&self) -> &'static str {
        "routes.json"
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    let mut proxy = SimpleProxy::new(args.port, Environment::Development);
    let logger = Logger::new();
    let router = Router::new(&Config());

    // Order matters
    proxy.add_middleware(Box::new(router));
    proxy.add_middleware(Box::new(logger));

    // Start proxy
    let _ = proxy.run().await;
}