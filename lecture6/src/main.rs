use clap::Parser;
use url::Url;
use smoltcp::phy::TunTapInterface;

mod dns;
mod ethernet;
mod http;

fn main() {
    let args = Args::parse();

    //let dns_server: std::net::Ipv4Addr = args.dns_server.parse().expect("error: unable to parse as a Ipv4 address");
    
    let url = Url::parse(&args.url).expect("error: unable to parse as a url");

    if url.scheme() != "http" {
        eprintln!("error: only HTTP protocol supported");
        return;
    }

    let tap = TunTapInterface::new(&args.tap_device).expect("error: unable to use as a network device");

    let domain_name = url.host_str().expect("domain name required");

    let addr = dns::resolve(args.dns_server).unwrap().unwrap();

    let mac = ethernet::MacAddress::new().into();

    http::get(tap, mac, addr, url).unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value = "1.1.1.1")]
    dns_server: String,

    #[clap(long)]
    url: String,

    #[clap(long)]
    tap_device: String,
}