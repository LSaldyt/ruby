use std::time::Duration;
use std::thread;
use std::str;

const delimeter : u8     = 10;
const header_len : usize = 6;

fn make_command(axis_index : u8, rot : f32) ->[u8; header_len] {
    // println!("{}", rot);
    // println!("{:?}", rot.to_be_bytes());
    // println!("{:?}", rot.to_le_bytes());
    let mut command : [u8; header_len] = [0; header_len];
    command[0] = 6;
    command[1..5].copy_from_slice(&rot.to_le_bytes());
    command[5] = delimeter;
    return command;
}

fn main() {
    let forward  = make_command(6,  90.0);
    let backward = make_command(6, -90.0);

    println!("Sending messages to /dev/ttyACM0");
    let dur = Duration::from_millis(100);
    let mut port = serialport::new("/dev/ttyACM0", 921600)
        .timeout(Duration::from_millis(100))
        .open().expect("Failed to open port");
    thread::sleep(Duration::from_millis(1000));
    let mut i : u32 = 0;
    let mut bytes_read = 0;
    loop {
        let mut serial_buf : Vec<u8> = vec![0; 512];
        bytes_read = match port.read(serial_buf.as_mut_slice()) {
            Ok(s)  => { println!("Read {} bytes:", s); s }
            Err(_) => 0
        };
        if bytes_read > 0 {
            let chars: Vec<char> = serial_buf.iter()
                .map(|b| *b as char)
                .filter(|c| *c != '\0')
                .collect::<Vec<_>>();
            let buf_str : String = chars.iter().collect::<String>();
            println!("{:?}", buf_str);
        }
        thread::sleep(Duration::from_millis(1000));
        if i % 2 == 0 {
            println!("forward!");
            port.write(&forward).expect("serial write failed");
        } else {
            println!("backward!");
            port.write(&backward).expect("serial write failed");
        }
        println!("sent output!");
        i += 1;
    }
}
