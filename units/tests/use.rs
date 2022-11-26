use units as u;

#[test]
fn create_quantity() {
    let meter = 1. * u::m;
    println!("{meter}");
}

#[test]
fn combine_units() {
    let area = 1. * u::m * u::m;
    println!("{area}");
}

#[test]
fn combine_quantities() {
    let length = 2. * u::m;
    let width = 4. * u::m;
    let area = length * width;
    println!("{area}");
}

#[test]
fn using_non_base_units() {
    let energy = 1. * u::J;
    println!("{energy}");
}
