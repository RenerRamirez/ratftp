use std::net::UdpSocket;

fn main() {
    let addr = "127.0.0.1:30000";
    let dest = "127.0.0.1:42069";
    let mut i: usize = 1;
    let socket = UdpSocket::bind(&addr).expect("Failed bind");
    println!("Bound to {addr}");

    for _x in 0..10 {
        /* OPCODE */
        let mut buf = [0; 32];
        buf[i] = 2;  
        i += 1;
        ////////////
        
        /* FILENAME */
        let filename_string = "hello.txt";
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

        socket.send_to(&buf, dest).expect("Failed send");

        /* WAITING FOR ACK */
        let mut reply_buf = [0; 32];
        let (_, _) = socket.recv_from(&mut reply_buf).expect("File received");
        println!("Sender received: {:?}", reply_buf);
        break;
    }
}
