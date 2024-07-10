pub mod circle {
    pub mod diff_circle;

    pub mod weak;
    pub mod weak_example;

    pub mod circle_reference;
}

pub mod table {
    pub mod map_lifetime;

    pub mod exercise_1;

    pub mod exercise_2;

    pub mod exercise_3;

    pub mod exercise_4;

    pub mod exercise_count_word;

    pub mod exercise_5;

    pub mod exercise_6;
}

pub mod convert {
    pub mod type_as;

    pub mod exercise_3;

    pub mod exercise_4;

    pub mod exercise_5;

    pub mod transmute;
}

pub mod oth_convert {

    pub mod exercise_1;

    pub mod exercise_2;

    pub mod exercise_3;
}

pub mod traitobj {
    pub mod advance_limited_grammer;
    pub mod advance_trait_generic_params;
    pub mod advance_trait_related;
    pub mod diff;
    pub mod dyn_stat_dispatch;
    pub mod full_limited_grammer;
    pub mod largest;
    pub mod newtype;
    pub mod restraint;
    pub mod same_method;
    pub mod super_traits;
    pub mod trait_safe;
    pub mod trait_safe_2;
    pub mod wrapper_fmt;
}

pub mod vector {
    pub mod dynamic_array;
    pub mod exercise_1;
    pub mod exercise_2;
    pub mod exercise_3;
    pub mod exercise_4;
    pub mod exercise_7;
    pub mod trait_objs_type_array;
    pub mod vec_method;
    pub mod vec_sort;
}

pub mod types {
    pub mod express;
    pub mod newtype;
    pub mod num;

    pub mod int2enum;
}

pub mod characters {
    pub mod strings;
}

pub mod from_into {
    pub mod exercise_1;

    pub mod exercise_2;

    pub mod exercise_3;

    pub mod exercise_4;
    pub mod exercise_5;
}

pub mod errors {
    pub mod exercise_1;
    pub mod exercise_2;

    pub mod exercise_res_1;
    pub mod exercise_res_2;
    pub mod exercise_res_3;
    pub mod exercise_res_4;
    pub mod exercise_res_5;
    pub mod exercise_res_6;
}

pub mod lifetime {
    pub mod reborrow;

    pub mod nll;

    pub mod reborrow_lt;

    pub mod exercise_2;

    pub mod exercise_5;

    pub mod exercise_6;
}

pub mod closure {
    pub mod cacher;
    pub mod fn_once;

    pub mod moves;

    pub mod fn_mut;

    pub mod exercise_1;
    pub mod exercise_2;

    pub mod exercise_3;

    pub mod exercise_4;

    pub mod exercise_5;

    pub mod exercise_6;

    pub mod exercise_9;

    pub mod exercise_10;

    pub mod exercise_10_1;

    pub mod exercise_11;
}

pub mod iterator {
    pub mod simulate_for;

    pub mod into_iter;

    pub mod counter;

    pub mod enumerate;
}

pub mod smart_ptr {
    pub mod deref;

    pub mod implicit_trans;

    pub mod diff_deref_convert;
}

pub mod reference {
    pub mod rc;

    pub mod cell;

    pub mod rc_many;
}

pub mod thread {
    pub mod barrier;
    pub mod example;
    pub mod example1;
}

pub mod channel {
    pub mod exercise;

    pub mod sender_many;
}

pub mod parallel {
    pub mod mutex;
}