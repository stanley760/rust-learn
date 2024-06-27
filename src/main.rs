use rust_learn::characters::strings;
use rust_learn::circle::diff_circle;
use rust_learn::table;
use rust_learn::traitobj::*;
use rust_learn::types;
use rust_learn::vector;
use rust_learn::convert;
use rust_learn::from_into;

fn main() {
    
    diff_circle::invoke();
    table::map_lifetime::invoke();
    table::exercise_1::invoke();
    table::exercise_2::invoke();
    table::exercise_3::invoke();
    table::exercise_4::invoke();
    table::exercise_5::invoke();
    table::exercise_6::invoke();
    table::exercise_count_word::invoke();
    advance_trait_generic_params::invoke();
    advance_limited_grammer::invoke();
    restraint::invoke();
    diff::invoke();
    dyn_stat_dispatch::invoke();
    largest::invoke();
    same_method::invoke();
    super_traits::invoke();
    trait_safe::invoke();
    wrapper_fmt::invoke();

    types::express::invoke();
    types::num::invoke();

    vector::dynamic_array::invoke();
    vector::exercise_1::invoke();
    vector::exercise_2::invoke();
    vector::exercise_3::invoke();
    vector::exercise_4::invoke();
    vector::exercise_7::invoke();
    vector::trait_objs_type_array::invoke();
    vector::vec_method::invoke();
    vector::vec_sort::invoke();

    strings::push_str();
    
    convert::type_as::invoke();
    convert::exercise_3::invoke();
    convert::exercise_4::invoke();
    convert::exercise_5::invoke();
    from_into::exercise_1::invoke();
    from_into::exercise_2::invoke();
    from_into::exercise_3::invoke();
    from_into::exercise_4::invoke();
    from_into::exercise_5::invoke();
}
