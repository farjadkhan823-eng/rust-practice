// #[derive(Debug)]
// struct StudentData {
//     name: String,
//     program: String,
//     semester: u32,
//     result: f32,
// }

pub fn data_types() {
    // // ARRAY //
    // let week: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    // println!(
    //     "first_day is: {} - last_day is: {}",
    //     week[0],
    //     week[week.len() - 1]
    // );
    // println!("Week days: {:?}", week);

    // // VECTOR //
    // let mut marks = Vec::new();
    // marks.push(10);
    // marks.push(20);
    // marks.push(30);
    // println!("marks: {:?}", marks);
    // marks.pop();
    // println!("marks: {:?}", marks);

    // // TUPPLE //
    // let employee: (&str, i32, i32) = ("Farjad", 20, 85000);

    // println!("name {}", employee.0);
    // println!("age {}", employee.1);
    // println!("salary {}", employee.2);

    // let (name, age, salary) = employee;

    // println!("name {}", name);
    // println!("age {}", age);
    // println!("salary {}", salary);

    // STRUCT //
    // let std_one = StudentData {
    //     name: String::from("Samad"),
    //     class: String::from("6th"),
    //     marks: 490,
    // };

    // println!("{:#?}", std_one);

    // STRUCT WITH VECTOR//
    // let mut student_list: Vec<StudentData> = Vec::new();

    // student_list.push(StudentData {
    //     name: String::from("Samad"),
    //     program: String::from("BS-Software Engineering"),
    //     semester: 5,
    //     result: 2.90,
    // });

    // student_list.push(StudentData {
    //     name: String::from("Farjad"),
    //     program: String::from("BS-Software Engineering"),
    //     semester: 3,
    //     result: 3.80,
    // });

    // student_list.push(StudentData {
    //     name: String::from("Arman"),
    //     program: String::from("BS-Computer Science"),
    //     semester: 4,
    //     result: 3.65,
    // });

    // println!("{:#?}", student_list);

    // IF-ELSE //
    // let age: i32 = 17;

    // if age >= 18 {
    //     println!("This person is valid for CNIC");
    // } else {
    //     println!("This person is not valid for CNIC");
    // }

    // // MATCH //

    // let day: i8 = 5;

    // match day {
    //     1 => println!("Monday"),
    //     2 => println!("Tuesday"),
    //     3 => println!("Wednesday"),
    //     4 => println!("Thursday"),
    //     5 => println!("Friday"),
    //     6 => println!("Saturday"),
    //     7 => println!("Sunday"),
    //     _ => println!("Invalid"),
    // };

    // let role: &str = "amdin";

    // match role {
    //     "admin" => println!("Dashboard"),
    //     "user" => println!("Profile"),
    //     _ => println!("Access Denied"),
    // };

    // LOOP //
    let mut i = 1;

    loop {
        println!("{}", i);

        i += 1;

        if i > 5 {
            break;
        }
    }
}
