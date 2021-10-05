fn main(){
    let b:u16 = 0xa726;
    let inverter:u16;

    inverter = !b;
    println!("~0x{:x} = 0x{:x}", b, inverter);
}
