use std::fs::File;
use std::io::Read;

mod hx;

fn main() {
    let mut fp = File::open("../test.bin").unwrap();
    let mut buf: [u8; hx::LINE_WIDTH] = [0; hx::LINE_WIDTH];
    let mut counter: usize = 0;

    println!("   Offset  ‖ Data");

    while fp.read(&mut buf).unwrap() > 0 {
        //let byte = byte_or_error.unwrap();
        let line = hx::Line {
            offset: counter,
            dump: buf,
        };
        counter += 1;
        line.print();
    }
}
