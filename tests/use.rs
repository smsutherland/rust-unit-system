use unit as u;

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
