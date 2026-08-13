mod avoiding_ownership;
mod borrow_operation;
mod module_one;
mod module_three;
mod module_two;
mod task_one;
mod task_two;
mod task_three;
mod task_four;
fn main() {
    module_one::module_one_func();
    module_two::module_two_func();
    module_three::ownership_rules();
    avoiding_ownership::avoid_owner();
    borrow_operation::borrow();
    task_one::find_vowels();
    task_two::find_first_letter();
    task_three::empty_check();
    task_four::borrowing();
}
