extern crate functional;

fn main() {
    functional::closures::run_without_closure();
    functional::closures::run_with_closure();

    functional::closures::fn_trait();
    functional::closures::fn_once_trait();
}
