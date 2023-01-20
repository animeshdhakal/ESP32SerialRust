use std::time::Duration;

enum SerialMessageType {
    BEGIN,
    MESSAGE,
    END,
}

#[derive(Debug)]
struct SerialMessageHeader {
    message_type: u32,
    message_length: u32,
}

impl SerialMessageHeader {
    fn from_bytes(bytes: &[u8; 8]) -> SerialMessageHeader {
        SerialMessageHeader {
            message_type: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            message_length: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        }
    }

    fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&self.message_type.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.message_length.to_le_bytes());
        bytes
    }
}

fn main() {
    let mut serial = serialport::new("/dev/ttyUSB0", 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap();

    let header = SerialMessageHeader {
        message_type: SerialMessageType::BEGIN as u32,
        message_length: 0,
    };

    let bytes = header.to_bytes();

    serial.write_all(&bytes).unwrap();

    loop {
        let mut buffer = [0u8; 8];
        match serial.read_exact(&mut buffer) {
            Ok(..) => {
                let header = SerialMessageHeader::from_bytes(&buffer);

                if header.message_type == SerialMessageType::MESSAGE as u32 {
                    let mut buffer = vec![0u8; header.message_length as usize];
                    serial.read_exact(&mut buffer).unwrap();
                    println!("{}", String::from_utf8_lossy(&buffer));
                }

                println!("{:?}", header);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => println!("{:?}", e),
        };
    }
}
