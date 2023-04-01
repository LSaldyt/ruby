use std::time::Duration;
use std::thread;
use std::str;

fn main() {
    println!("Sending messages to /dev/ttyACM0");
    let dur = Duration::from_millis(100);
    let mut port = serialport::new("/dev/ttyACM0", 921600)
        .timeout(Duration::from_millis(100))
        .open().expect("Failed to open port");
    thread::sleep(Duration::from_millis(1000));
    let mut bytes_read = 0;
    // let output = "Command: ax1 90 deg".as_bytes();
    let output = "abc\n".as_bytes();
    println!("{:?}", output);
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
        port.write(output).expect("serial write failed");
        println!("sent output!");
    }
}
