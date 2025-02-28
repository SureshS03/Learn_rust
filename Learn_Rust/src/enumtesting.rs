#[warn(unused_mut)]

// Enum is a type that can be one of several variants
//create a enum structure
enum TestByEnum {
    Name(String),
    Age(u32),
    GameSkills(i32),
    Rank(String),
    Aim{
        xaxis:u32,
        yaxis:u32,
    },
}

//do some function with created enum
fn test(evn: TestByEnum) {
    //println!("Test by Enum");
    match evn {
        TestByEnum::Name(c) => println!("Name is {}",c),
        TestByEnum::Age(var_age) => {
                        println!("Age is {}", var_age);
                        println!("Age in 2 times is {}", var_age);
            },
        TestByEnum::GameSkills(var_skills_rate) => {
                println!("Your Gaming Skills in -10 to 10");
                println!("{}",var_skills_rate)
            },
        TestByEnum::Rank(var_rank) => println!("Your Rank is {}",var_rank),
        TestByEnum::Aim { xaxis, yaxis } => {
            let mut addedaim = xaxis + yaxis;
            println!("Your Aim as added {}",addedaim)
        }
    }
}

//main function and pass the values of enums, to that functions to work on it
fn main () {
    let pass_name = TestByEnum::Name("Suresh".to_owned());
    let pass_age = TestByEnum::Age(19);
    let pass_game = TestByEnum::GameSkills(-8);
    let pass_rank = TestByEnum::Rank("Gold".to_owned());
    let pass_aim = TestByEnum::Aim { xaxis: 3, yaxis: 4 };

    test(pass_name);
    test(pass_age);
    test(pass_game);
    test(pass_rank);
    test(pass_aim);
}
