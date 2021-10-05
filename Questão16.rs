fn main(){
    let a:u16 = 0x6db7;
    let b:u16;
    let mascara:u16 = 0xff00;

    b = a^mascara;
    println!("0x{:x} = 0x{:x} ^ 0x{:x}", b, a, mascara);
}
