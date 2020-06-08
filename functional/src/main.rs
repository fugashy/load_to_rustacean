extern crate functional;

fn main() {
    functional::closures::run_without_closure();
    functional::closures::run_with_closure();
    functional::closures::run_with_closure_improved();

    functional::closures::fn_trait();
    functional::closures::fn_move();

    functional::iterators::simple_iteration();
    functional::iterators::next();
    functional::iterators::sum();
    functional::iterators::map();
    functional::iterators::filter();

    functional::iterators::call_next_directly();
    functional::iterators::using_other_iterator_trait_methods();
}
