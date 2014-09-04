
//Magical way to transpose 64x64 bit matrix.
//More: http://www.hackersdelight.org/hdcodetxt/transpose8.c.txt
pub fn transpose(x: u64) -> u64 {
    let t = (x ^ (x >> 7)) & 0x00AA00AA00AA00AAu64;
    let x = x ^ t ^ (t << 7);
    let t = (x ^ (x >> 14)) & 0x0000CCCC0000CCCCu64;
    let x = x ^ t ^ (t << 14);
    let t = (x ^ (x >> 28)) & 0x00000000F0F0F0F0u64;
    let x = x ^ t ^ (t << 28);
    x
}