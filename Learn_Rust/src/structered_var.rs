fn main () {
    let (a,b) = (2,3);
    assert_eq!(a,2);
    assert_eq!(b,3);
    println!("we used tuple as in structed partten to assign var");
    //if we want to change the value means we can use "mut" key word
    let (mut c, _d) = (3,4); //used underscore to avoid unused var, or else we can use "#[allow(unused_variables)]" too
    c += 2;
    assert_eq!(c, 5);
    println!("working good");
    test();
}

fn test () {
    let (x,y,z); // this is destructor, using tuple bcz we can use any data type, use array for same data type
    (x,..) = (1,2,3); // x = 1, the place or index of x is going to assign the value in RHS of place of x
    (..,y) = (2,90,("bro","yoo",10)); // cant use diff data type in array so use tuple 
    [..,z] = [1,2,69]; //z = 69, z is in last so that the last value of the tuple is going to assign, array will work here bcz we used same type and also remeber if we use array we must have to assign LHS and RHS both as array
    //(..,z) = [2,4,"hello"]; // give error bcz we used diff data type
    println!("{} {:?} {}",x,y,z)
}