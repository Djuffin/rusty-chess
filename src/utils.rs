
//Magical way to transpose 64x64 bit matrix.
//More: http://www.hackersdelight.org/hdcodetxt/transpose8.c.txt
#[inline]
pub fn transpose(x: u64) -> u64 {
    let t = (x ^ (x >> 7)) & 0x00AA00AA00AA00AAu64;
    let x = x ^ t ^ (t << 7);
    let t = (x ^ (x >> 14)) & 0x0000CCCC0000CCCCu64;
    let x = x ^ t ^ (t << 14);
    let t = (x ^ (x >> 28)) & 0x00000000F0F0F0F0u64;
    let x = x ^ t ^ (t << 28);
    x
}

//Magical way to reverse bits in one byte
//SLOW. use tables version instead
#[inline]
pub fn reverse(x:u8) -> u8 {
    let r = ((x as u64) * 0x0202020202u64 & 0x010884422010u64) % 1023;
    r as u8
}

pub fn write_to_log(line: String) {
    use std::fs::OpenOptions;
    use std::io::Write;
    let mb_file = OpenOptions::new().read(true).write(true)
            .append(true).open("rchess.log");
    let mut file = match mb_file {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };
    let _ = writeln!(&mut file, "{}", line);
}
