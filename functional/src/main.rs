extern crate functional;

fn main() {
    functional::closures::run_without_closure();
    functional::closures::run_with_closure();
    functional::closures::run_with_closure_improved();

    functional::closures::several_ways_to_describe_closures();
    functional::closures::capture_values_as_const_one();
    functional::closures::capture_values_in_whole();

    functional::iterators::simple_iteration();
    functional::iterators::next();
    functional::iterators::sum();
    functional::iterators::map();
    functional::iterators::filter();

    functional::iterators::call_next_of_counter();
    functional::iterators::using_other_iterator_trait_methods();
    functional::iterators::double_iteration();
}
