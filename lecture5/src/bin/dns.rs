//cargo run --bin dns -q -- --domain_name www.rustinaction.com
use std::{error::Error, net::{SocketAddr, UdpSocket}, time::Duration};

use clap::Parser;
use trust_dns::{op::{Message, Query, OpCode}, rr::{RecordType, Name}, serialize::binary::{BinEncodable, BinEncoder}};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let domain_name = Name::from_ascii(args.domain_name).expect("invalid domain name");
    let dns_server: SocketAddr = format!("{}:53", args.dns_server).parse().expect("invalid address");

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
       .set_message_type(trust_dns::op::MessageType::Query)
       .add_query(Query::query(domain_name, RecordType::A))
       .set_op_code(OpCode::Query)
       .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    localhost.send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured");

    localhost.recv_from(&mut response_as_bytes)
        .expect("timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes)
        .expect("unable to parse response");

    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let ip = resource
                .to_ip_addr()
                .expect("invalid IP address received");
            
            println!("{}", ip);
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 's', long, default_value = "1.1.1.1")]
    dns_server: String,

    #[clap(long = "domain_name")]
    domain_name: String,
}