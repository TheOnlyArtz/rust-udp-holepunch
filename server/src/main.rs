use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9999")?;
    let mut peers: Vec<String> = vec![];

    loop {
        let mut buf = [0; 1024];
        let (_, src) = socket.recv_from(&mut buf)?;
        let stringified_buff = String::from_utf8(buf.to_vec()).unwrap();
        let stringified_buff = stringified_buff.trim_matches(char::from(0));
    
        println!("[NEW MESSAGE]{:?} => {:?}", src, stringified_buff);

        if stringified_buff != "register" {
            continue
        }

        if !peers.contains(&format!("{}", src)) {
            peers.push(format!("{}", src));
        }

        for p in &peers {
            let filtered_peers = filter_peers(&peers, p);

            if filtered_peers.len() > 0 {
                socket.send_to(filtered_peers.join(",").as_bytes(), p)?;
            }
        }
    }
}

fn filter_peers(peers: &Vec<String>, filter: &String) -> Vec<String> {
    let mut new_peers: Vec<String> = vec![];

    for p in peers {
        if p != filter {
            new_peers.push(String::from(p));
        }
    }

    new_peers
}