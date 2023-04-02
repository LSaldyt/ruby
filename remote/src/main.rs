use std::time::Duration;
use std::thread;
use std::str;

const DELIMITER : u8     = 10;
const HEADER_LEN : usize = 10;

fn make_command(command_index: u32, axis_index : u8, rot : f32) ->[u8; HEADER_LEN] {
    let mut command : [u8; HEADER_LEN] = [0; HEADER_LEN];
    command[0..4].copy_from_slice(&command_index.to_le_bytes());
    command[4] = axis_index;
    command[5..9].copy_from_slice(&rot.to_le_bytes());
    command[9] = DELIMITER;
    return command;
}

fn main() {
    let forward  = make_command(1, 6,  60.0);
    let backward = make_command(2, 6, -60.0);

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
        thread::sleep(Duration::from_millis(5000));
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
