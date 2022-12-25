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
fn using_to() {
    let distance = 12. * u::cm;
    let time = 2. * u::s;
    let velocity1 = distance / time;
    let velocity2 = 0.12 / 2. * u::m / u::s;
    assert_eq!(velocity1.to(u::m / u::s), velocity2);
}
