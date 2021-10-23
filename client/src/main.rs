use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::{env, thread, time::Duration};
use std::sync::mpsc::{channel, Sender};

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();
    let local_address = args.next();
    let rendezvous_address = args.next();
    if local_address.is_none() || rendezvous_address.is_none() {
        return Ok(());
    }

    let rendezvous_address = rendezvous_address.unwrap();
    let rendezvous_address = rendezvous_address.parse::<IpAddr>().unwrap();
    let rendezvous = SocketAddr::new(rendezvous_address, 9999);
    let local_address: String = local_address.unwrap();

    let socket = UdpSocket::bind(local_address)?;

    let register_socket = socket.try_clone().unwrap();
    let register_handle = thread::spawn(move || {
        let msg = register_socket.send_to(b"register", rendezvous);
        if msg.is_err() {
            println!("{}", msg.err().unwrap());
            return;
        }

        println!("Sent {} bytes to: {}", msg.unwrap(), rendezvous);
    });

    register_handle.join().unwrap();

    let (chatter_sender, chatter_receiver) = channel::<(Vec<u8>, String)>();

    let worker_socket = socket.try_clone().unwrap();
    let worker_handle = thread::spawn(move || {
        loop {
            let recv_msg = chatter_receiver.recv();
    
            if recv_msg.is_err() {
                println!("There was an error recv from channel");
                return;
            }
    
            let recv_msg = recv_msg.unwrap();
    
            let sent = worker_socket.send_to(&recv_msg.0, &recv_msg.1);
            
            if sent.is_err() {
                println!("There was an error sending to {}", recv_msg.1);
                return;
            }    
        }
    });

    let chatter_sender_clone = chatter_sender.clone();
    listen_for_conns(socket, chatter_sender_clone)?;

    worker_handle.join().unwrap();


    Ok(())
}

fn listen_for_conns(local_socket: UdpSocket, sender: Sender<(Vec<u8>, String)>) -> std::io::Result<()> {
    loop {
        let mut buf = [0; 1024];
        let msg = local_socket.recv_from(&mut buf);

        if msg.is_err() {
            continue;
        }

        let (sz, addr) = msg?;

        let buf = String::from_utf8(buf[..sz].to_vec()).unwrap();
        println!("Recieved {} from {}", buf, addr);

        if buf == "handshake!" {
            continue;
        }

        let peers = buf.split(",");

        for p in peers {
            let connect_to = String::from(p);
            let cloned_sender = sender.clone();
            
            thread::spawn(move || loop {
                cloned_sender.send((b"handshake!".to_vec(), String::from(&connect_to))).unwrap();

                thread::sleep(Duration::from_millis(5000));
            });
        }
    }
}