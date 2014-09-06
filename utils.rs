
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
#[inline]
pub fn reverse(x:u8) -> u8 {
    //TODO: we should compare perf with table lookup
    let r = ((x as u64) * 0x0202020202u64 & 0x010884422010u64) % 1023;
    r as u8
}