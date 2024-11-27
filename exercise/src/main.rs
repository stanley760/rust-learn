use exercise::characters::strings;
use exercise::circle::diff_circle;
use exercise::closure;
use exercise::convert;
use exercise::errors;
use exercise::from_into;
use exercise::lifetime;
use exercise::oth_convert;
use exercise::table;
use exercise::traitobj::*;
use exercise::types;
use exercise::vector;
use exercise::channel;

fn main() {
    channel::sender_many::invoke();
    channel::exercise::invoke();
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

    diff::invoke();
    largest::invoke();
    restraint::invoke();
    types::num::invoke();
    trait_safe::invoke();
    same_method::invoke();
    wrapper_fmt::invoke();
    super_traits::invoke();
    types::express::invoke();
    dyn_stat_dispatch::invoke();

    vector::vec_sort::invoke();
    vector::exercise_1::invoke();
    vector::exercise_2::invoke();
    vector::exercise_3::invoke();
    vector::exercise_4::invoke();
    vector::exercise_7::invoke();
    vector::vec_method::invoke();
    vector::dynamic_array::invoke();
    vector::trait_objs_type_array::invoke();

    strings::push_str();

    convert::type_as::invoke();
    convert::exercise_3::invoke();
    convert::exercise_4::invoke();
    convert::exercise_5::invoke();
    convert::transmute::invoke();
    from_into::exercise_1::invoke();
    from_into::exercise_2::invoke();
    from_into::exercise_3::invoke();
    from_into::exercise_4::invoke();
    from_into::exercise_5::invoke();
    oth_convert::exercise_1::invoke();
    oth_convert::exercise_2::invoke();
    oth_convert::exercise_3::invoke();

    errors::exercise_2::invoke();
    errors::exercise_res_1::invoke();
    errors::exercise_res_2::invoke();
    errors::exercise_res_3::invoke();
    errors::exercise_res_4::invoke();
    errors::exercise_res_5::invoke();
    errors::exercise_res_6::invoke();
    errors::dyn_errors::test_dyn_error();
    lifetime::reborrow::invoke();
    lifetime::reborrow_lt::invoke();
    lifetime::exercise_5::invoke();
    lifetime::exercise_6::invoke();

    closure::cacher::invoke();
    closure::fn_once::invoke();
    closure::moves::invoke();
    closure::fn_mut::invoke();


}
