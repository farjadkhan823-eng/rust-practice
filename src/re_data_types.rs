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

    // TUPPLE //
    let employee: (&str, i32, i32) = ("Farjad", 20, 85000);

    println!("name {}", employee.0);
    println!("age {}", employee.1);
    println!("salary {}", employee.2);

    let (name, age, salary) = employee;

    println!("name {}", name);
    println!("age {}", age);
    println!("salary {}", salary);
}
