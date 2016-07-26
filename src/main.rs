extern crate dns_parser;

use std::net::UdpSocket;
use dns_parser::Packet;

fn main() {

    let query_id = 5;

    let mut builder = dns_parser::Builder::new_query(query_id, true);
    builder.add_question("cardin.email",
                         dns_parser::QueryType::TXT,
                         dns_parser::QueryClass::IN);

    let mut result = builder.build().expect("build failed");
    let mut socket = UdpSocket::bind("0.0.0.0:34254").expect("sock fails");
    let ip = std::net::Ipv4Addr::new(208, 67, 222, 222);
    let sockaddr = std::net::SocketAddrV4::new(ip, 53);

    let buf = &mut result[..];
    socket.send_to(buf, &sockaddr);

    let mut buf2 = [0; 1000];
    let (amt, src) = socket.recv_from(&mut buf2).expect("cvfrm failed");


    let packet = Packet::parse(&buf2).unwrap();


    println!("query id {}", packet.header.id);
    println!("questions len {}", packet.questions.len());
    println!("answers len {}", packet.answers.len());
    println!("additional len {}", packet.additional.len());

    for a in packet.answers {
        println!("name: {}", a.name);

        match a.data {
            dns_parser::RRData::Unknown(data) => {
                let mut vec1 = Vec::new();
                for i in data.iter() {
                    vec1.push(*i);
                }
                println!("value: {}", std::string::String::from_utf8(vec1).unwrap())
            }
            _ => println!("value"),
        }

    }

}
