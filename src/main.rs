mod avoiding_ownership;
mod module_one;
mod module_three;
mod module_two;
fn main() {
    module_one::module_one_func();
    module_two::module_two_func();
    module_three::ownership_rules();
    avoiding_ownership::avoid_owner();
}
