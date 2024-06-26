use rust_learn::circle::diff_circle;
use rust_learn::table:: map_lifetime;
use rust_learn::traitobj::*;

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
}
