fn main(){
    let a:u16 = 0b0110110110110111;
    let b:u16;

    b = a >> 6;
    println!("b = 0x{:x}", b);
}
