use std::net::UdpSocket;
use std::fs;

fn main() {
    let addr = "127.0.0.1:30000";
    let dest = "127.0.0.1:42069";
    let mut i: usize = 1;
    let socket = UdpSocket::bind(&addr).expect("Failed bind");
    println!("Bound to {addr}");

    /* OPCODE */
    let mut buf = [0; 64];
    buf[i] = 2;
    i += 1;
    ////////////

    /* FILENAME */
    let filename_string = "/home/rener/bevy/CHANGELOG.md";
    for c in filename_string.chars() {
        buf[i] = c as u8;
        i += 1;
    }
    buf[i] = 0;
    i += 1;
    /////////////

    /* MODE */
    let _netascii_string = "netascii";
    let octet_string = "octet";
    let _mail_string = "mail";
    for c in octet_string.chars() {
        buf[i] = c as u8;
        i += 1;
    }
    buf[i] = 0;
    ////////////

    /* SEND TO SERVER */
    socket.send_to(&buf, dest).expect("Failed send");

    /* WAITING FOR ACK */
    let mut reply_buf = [0; 128];
    let (_, _) = socket.recv_from(&mut reply_buf).expect("File received");
    // println!("Sender received: {:?}", reply_buf);

    if reply_buf[1] == 4 {
        println!("ACK'd");
    }

    let file_in_bytes = fs::read(filename_string).expect("File in bytes failed");
    println!("File size in bytes: {}", file_in_bytes.len());

    let mut i: u16 = 1;
    for chunk in file_in_bytes.chunks(512) {
        let mut data_buf = [0; 520];
        data_buf[1] = 3;
        data_buf[2] = i.to_be_bytes()[0];
        data_buf[3] = i.to_be_bytes()[1];
        let mut y = 5;
        for c in chunk.iter() {
            data_buf[y] = *c;
            y += 1;
        }
        socket.send_to(&data_buf, dest).expect("Failed send");
        let (_, _) = socket.recv_from(&mut reply_buf).expect("File received");
        // println!("data buf: {:?}", data_buf);
        i += 1;
    }
}
