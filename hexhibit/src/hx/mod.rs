pub const LINE_WIDTH : usize = 32;
const VERTICAL_SEPARATOR: char = '‖';
pub struct Line {
    pub offset: usize,
    pub dump: [u8; LINE_WIDTH],
}

impl Line {
    pub fn print(&self) -> () {
        print!("0x{:08X} {} ", self.offset, VERTICAL_SEPARATOR);

        for byte in self.dump {
            print!("{:02X} ", byte);
        }

        print!(" {} ", VERTICAL_SEPARATOR);
        for byte in self.dump {
            if byte.is_ascii_alphanumeric() {
                print!("{}", byte as char);
            } else {
                print!("▯");
            }
        }

        println!("");
    }
}

impl Default for Line {
    fn default() -> Line {
        Line {
            offset: 0,
            dump: [0; LINE_WIDTH],
        }
    }
}
