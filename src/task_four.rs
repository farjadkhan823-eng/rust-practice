pub fn borrowing() {
    let city: String = String::from("Karachi");
    let copy_city: String = city_func(&city);
    println!("original {} and copy {}", city, copy_city);
}

fn city_func(city: &String) -> String {
    return city.to_string();
}
