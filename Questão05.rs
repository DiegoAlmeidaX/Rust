fn main(){
    let var:i32 = 0x5b3c;
    let inverter:i32;

    inverter = !var;
    println!("~0x{:x} = 0x{:x}", var, inverter);
}
