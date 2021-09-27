use crate::trait_params::{print_info, Circle, Square};

mod trait_params;

fn main() {
    let c = Circle{radius: 6.0};
    print_info(c);

    let s = Square{side: 20.0};
    print_info(s);
}
