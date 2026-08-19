pub fn mutable_borrow(){
    let mut status:String = String::from("5PM");
    println!("Attendence In {}",status);
    borrow(&mut status);
    println!("Attendence Out {}",status);
}

fn borrow(att: &mut String){
   att.push_str(" - 7PM");
}