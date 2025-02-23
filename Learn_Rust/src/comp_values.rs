pub fn test() {
    let bi = 0b0001; //0b = binary, 0001 = 8 4 2 1
    let dec = 2_224; // this is deicamal the "_" is for better reading only
    let octa = 0o0011; // 0o = octal, 0001 = (0 * 8^3) + (...) + (...) + (1 * 8^0)
    let hex = 0x0011; // 0x = hexa decimal

    //we can extaned upto how many bits that varibale can hold like i32, i64, i128, i256 etc..

    println!("this is binary {}",bi);
    println!("this is decimal {}",dec);
    println!("this is octa {}",octa);
    println!("this is hexa deicaml {}",hex);
}