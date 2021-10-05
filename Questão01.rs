fn main(){
    let var:u16 = 0x7ff;
    let inverter:u16;

    inverter = !var;
    println!("~0x{:x} = 0x{:x}",var, inverter);
}
