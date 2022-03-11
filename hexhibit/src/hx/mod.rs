use std::fs::File;
use std::io::Read;

// Number of bytes displayed on a line
const HX_LINE_WIDTH : usize = 32;

// Column separator (UTF-8: U+0x2016)
const HX_VERTICAL_SEPARATOR: char = '‖';

// Non-printable character (UTF-8: U+0x25AF)
const HX_UNKNOWN: char = '▯';

// Describes a line in output
struct Line {
    // Offset in file (multiple of HX_LINE_WIDTH)
    offset: usize,

    // Bytes in the line
    dump: [u8; HX_LINE_WIDTH],
}

impl Line {
    // Print a line
    fn print(&self) -> () {

        // Write on the left
        print!("0x{:08X} {} ", self.offset, HX_VERTICAL_SEPARATOR);

        // Write bytes
        for byte in self.dump {
            print!("{:02X} ", byte);
        }

        print!(" {} ", HX_VERTICAL_SEPARATOR);

        // Write the corresponding ASCII character if it's printable
        for byte in self.dump {
            let ascii = if byte.is_ascii_alphanumeric() { byte as char } else { HX_UNKNOWN };
            print!("{}", ascii);
        }

        println!("");
    }
}

pub fn dump(filename: &String) {

    let mut file = File::open(filename).unwrap();
    let mut line_buffer: [u8; HX_LINE_WIDTH] = [0; HX_LINE_WIDTH];
    let mut offset_counter: usize = 0;

    // Read the file by chunk of HX_LINE_WIDTH bytes
    while file.read(&mut line_buffer).unwrap() > 0 {

        // Build a line with current offset and data
        let line = Line {
            offset: offset_counter * HX_LINE_WIDTH,
            dump: line_buffer,
        };

        // Update counter and print the current line
        offset_counter += 1;
        line.print();
    }
}
