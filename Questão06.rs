fn main(){
    let a:u16 = 0x6db7;
    let inverter:u16;
    
    inverter = !a;
    println!("~0x{:x} = 0x{:x}", a, inverter);
}
