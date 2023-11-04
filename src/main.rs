use std::net::UdpSocket;

#[derive(Debug)]
enum OpCode {
    RRQ,
    WRQ,
    Data,
    Ack,
    Error,
}

#[derive(Debug)]
enum Mode {
    NetAscii,
    Octet,
    Mail
}

fn main() {
    let addr = "127.0.0.1:42069";
    let socket = UdpSocket::bind(&addr).expect("Failed bind");
    println!("Bound to {addr}");

    for i in 0..2 {
        let mut buf = [0; 32];
        let (amt, src) = socket.recv_from(&mut buf).expect("Failed receive");

        println!("Received {amt} bytes from {:?}: {:?}", src, buf);

        let index_of_opcode = 1; // only the first byte contains the value
        let real_opcode: OpCode; 
        match buf[index_of_opcode] as u16 {
            1 => {
                real_opcode = OpCode::RRQ;
                break;
            },
            2 => { 
                real_opcode = OpCode::WRQ;
            },
            3 => { 
                real_opcode = OpCode::Data;
                break;
            },
            4 => {
                real_opcode = OpCode::Ack;
                break;
            },
            5 => { 
                real_opcode = OpCode::Error;
                break;
            },
            _ => { 
                real_opcode = OpCode::Error;
            },
        }
        println!("Packet: {:?}", real_opcode);

        /* EXTRACT FILENAME */
        let index_of_filename = buf[index_of_opcode..]
            .iter()
            .position(|&c| c == 0)
            .unwrap() + index_of_opcode;
        let filename = String::from_utf8(buf[2..index_of_filename].to_vec()).unwrap(); 

        println!("Filename: {filename}");

        let index_of_mode = buf[(index_of_filename+1)..]
            .iter()
            .position(|&c| c == 0)
            .unwrap() + index_of_filename + 1;

        let mode_from_pocket = String::from_utf8(buf[(index_of_filename+1)..index_of_mode].to_vec()).unwrap().to_lowercase();

        let octet_string = String::from_utf8("octet".into()).unwrap();
        let netascii_string = String::from_utf8("netascii".into()).unwrap();
        let mail_string = String::from_utf8("mail".into()).unwrap();

        let real_mode: Mode;

        if mode_from_pocket == netascii_string {
            real_mode = Mode::NetAscii;
        } else if mode_from_pocket == octet_string {
            real_mode = Mode::Octet;
        } else if mode_from_pocket == mail_string {
            real_mode = Mode::Mail;
        } else {
            real_mode = Mode::Octet;
        }
        
        println!("Mode: {:?}", real_mode);

        let mut reply_buf = [0; 32];
        reply_buf[1] = 4;
        reply_buf[3] = i;
        socket.send_to(&reply_buf, src).expect("Failed send");
    }
}
