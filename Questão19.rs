fn main(){
    let m1:u16 = 0xe3;
    let m2:u16 = 0x14;
    let mut b:u16 = 0xaf;
    let aux:u16;

    aux = b;
    b = (b&m1)|m2;
    println!("0x{:x} = (0x{:x} & 0x{:x}) | 0x{:x}", b, aux, m1, m2);
}
