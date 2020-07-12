extern crate pattern_matching;

fn main() {
    pattern_matching::simple_matching();
    pattern_matching::all_patterns_must_be_covered();
    pattern_matching::if_let_notation();
    pattern_matching::all_expressions_must_return_the_same_typed_value();

    pattern_matching::mixed_if_statements();
    pattern_matching::pop_as_long_as_vector_has_values();
    pattern_matching::for_statement_can_use_patterns();
    pattern_matching::let_statement_uses_pattern();

    pattern_matching::various_refutables();
    pattern_matching::match_arm_begins_new_scope();
    pattern_matching::multiple_match();
    pattern_matching::range_match();
    pattern_matching::decomposition_and_matching_of_struct_may_be_tricky();
    pattern_matching::destructure_enum();
    pattern_matching::destructure_nested_structures();

    pattern_matching::wild_card_in_the_nest();
    pattern_matching::difference_between_unused_value_and_wild_card();
    pattern_matching::ignore_a_range_of_structure();
    pattern_matching::ignore_a_range_of_tuple_and_list();

    pattern_matching::at_binding();
}
