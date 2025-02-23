fn main () {
    let x: i32 = 5;

    let msg: &str = "suresh";
    { //inner scope 
        let x: i32 = 12;
        assert_eq!(x,12);   // this value of x is 12 bc its innside the inner scope
        println!("{}",msg);  //gobal var can access inside the inner scope 
    }
    assert_eq!(x,5);// check if x and 5 are eqaul , or throw error
    println!("done");
}
