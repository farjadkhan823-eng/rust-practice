pub fn data_types() {
    // VECTOR //
    let mut marks = Vec::new();
    marks.push(10);
    marks.push(20);
    marks.push(30);
    println!("marks: {:?}", marks);
    marks.pop();
    println!("marks: {:?}", marks);

    let week: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    println!("first_day is: {} - last_day is: {}", week[0], week[week.len() - 1]);
    println!("Week days: {:?}", week);
}
