use rust_learn::circle::diff_circle;
use rust_learn::table::map_lifetime;
use rust_learn::traitobj::*;
use rust_learn::types::*;
use rust_learn::vector::*;

fn main() {
    diff_circle::invoke();
    map_lifetime::invoke();
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
    
    express::invoke();
    num::invoke();

    dynamic_array::invoke();
    exercise_1::invoke();
    exercise_2::invoke();
    exercise_3::invoke();
    exercise_4::invoke();
    exercise_7::invoke();
    exercise_8::invoke();
    trait_objs_type_array::invoke();
    vec_method::invoke();
    vec_sort::invoke();
}
