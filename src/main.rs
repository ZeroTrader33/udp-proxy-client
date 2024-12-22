
fn main() {
    std::panic::set_hook(Box::new(|info| {
        println!("Caught a panic: {:?}", info);
    }));
    udp_proxy::UdpProxyClient::run();
}
