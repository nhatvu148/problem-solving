mod components;
use std::rc::Rc;

use components::comp_1::*;

fn main() {
    let t: Box<dyn Vehicle>; // variable of a trait type that cannot
                             // be computed at compile time
    t = Box::new(Truck);
    t.drive();

    let (truck_a, truck_b, truck_c) = (
        Rc::new(Truck3 { capacity: 1 }),
        Rc::new(Truck3 { capacity: 2 }),
        Rc::new(Truck3 { capacity: 3 }),
    );

    let facility_one = vec![Rc::clone(&truck_a), Rc::clone(&truck_b)];
    let facility_two = vec![Rc::clone(&truck_b), Rc::clone(&truck_c)];

    println!("Facility one {:?}", facility_one);
    println!("Facility two {:?}", facility_two);
    println!("Truck b strong count {:?}", Rc::strong_count(&truck_b));

    std::mem::drop(facility_two);
    println!("Facility one after drop {:?}", facility_one);
    println!("Truck b strong count {:?}", Rc::strong_count(&truck_b));
}
