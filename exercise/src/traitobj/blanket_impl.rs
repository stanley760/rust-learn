#[allow(dead_code)]
trait TraitA {}
#[allow(dead_code)]
trait TraitB {}

impl<T: TraitA> TraitB for T {} // first implementation here

impl TraitA for i32 {}
// impl TraitB for i32 {    // ❌ conflicting implementation for `i32`

// }
