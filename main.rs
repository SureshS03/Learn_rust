fn main () {
    println!("say hello");
    muteablevar();
}

fn shadowning () {
    let x: i32 = 5; 
    let x: &str = "gello"; // shadowning the var x with new value 
    print!("{} this is value of",x);
}

fn eqaul () {
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

fn declare () {
    let x = 23;
    let _y: &str; // we can declare un installied var by using _ before it (with the var type)
    print!("{}",x)
}

fn muteablevar() {
    let mut x = 5; // by deafult rust var are immutebale (cant change) but by use mut we can change the value of var
    println!("{}",x);
    x += 5;
    println!("{}",x);
}