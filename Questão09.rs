fn main(){
    let a:u16 = 0x6db7;
    let b:u16 = 0xa726;
    let xor:u16;

    xor = a^b;
    println!("0x{:x} ^ 0x{:x} = 0x{:x}", a, b, xor);
}
