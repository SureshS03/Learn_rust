fn muteablevar() {
    let mut x = 5; // by deafult rust var are immutebale (cant change) but by use mut we can change the value of var
    println!("{}",x);
    x += 5;
    println!("{}",x);
}