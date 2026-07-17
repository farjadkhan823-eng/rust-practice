pub fn module_one_func() {
    let string_literal: String = String::from("rust chapter 2");
    println!("our first string is {}", string_literal);

    let mut num: i16 = 10;
    println!("num is {}", num);
    num = 32767;
    println!("num is {}", num);

    let name: &str = "farjad";
    println!("num is {}", name);

    let info: (&str, u8) = ("farjad", 20);
    let (my_name, my_age) = info;
    println!("name {} age {}", my_name, my_age);

    struct User {
        name: String,
        age: u32,
        is_student: bool,
    }

    let user = User {
        name: String::from("Farjad"),
        age: 20,
        is_student: true,
    };

    println!(
        "name: {} age: {} today_present: {}",
        user.name, user.age, user.is_student
    )
}
