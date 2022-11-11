#![allow(dead_code, unused_imports)]
use std::sync::Arc;
use std::{rc::Rc, thread::spawn};

pub trait Vehicle {
    fn drive(&self);
}

pub struct Truck;
pub struct Truck2 {
    next_truck: Option<Box<Truck2>>, // recursive data type
}
#[derive(Debug)]
pub struct Truck3 {
    pub capacity: i32,
}

impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck is driving");
    }
}

pub fn smart_pointer_example() {
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

pub fn smart_pointer_thread_example() {
    let t: Box<dyn Vehicle>; // variable of a trait type that cannot
                             // be computed at compile time
    t = Box::new(Truck);
    t.drive();

    let (truck_a, truck_b, truck_c) = (
        Arc::new(Truck3 { capacity: 1 }),
        Arc::new(Truck3 { capacity: 2 }),
        Arc::new(Truck3 { capacity: 3 }),
    );

    let thread = std::thread::spawn(move || {
        let facility_one = vec![Arc::clone(&truck_a), Arc::clone(&truck_b)];
        let facility_two = vec![Arc::clone(&truck_b), Arc::clone(&truck_c)];
        (facility_one, facility_two)
    });

    let (facility_one, facility_two) = thread.join().unwrap();

    println!("Facility one {:?}", facility_one);
    println!("Facility two {:?}", facility_two);

    let truck_b = Arc::clone(&facility_one[1]);
    println!("Truck b strong count {:?}", Arc::strong_count(&truck_b));

    std::mem::drop(facility_two);
    println!("Facility one after drop {:?}", facility_one);
    println!("Truck b strong count {:?}", Arc::strong_count(&truck_b));
}

pub trait Speaks {
    fn speak(&self);
}

pub trait Animal {
    fn animal_type(&self) -> &str;
    fn noise(&self) -> &str;
}

impl<T> Speaks for T
where
    T: Animal,
{
    fn speak(&self) {
        println!("The {} said {}", self.animal_type(), self.noise());
    }
}

pub struct Dog {}
pub struct Cat {}

impl Animal for Dog {
    fn animal_type(&self) -> &str {
        "dog"
    }

    fn noise(&self) -> &str {
        "woof"
    }
}

impl Animal for Cat {
    fn animal_type(&self) -> &str {
        "cat"
    }

    fn noise(&self) -> &str {
        "meow"
    }
}

trait Human {
    fn name(&self) -> &str;
    fn sentence(&self) -> &str;
}

struct Person {}

impl Human for Person {
    fn name(&self) -> &str {
        "man"
    }

    fn sentence(&self) -> &str {
        "hello"
    }
}

impl Speaks for dyn Human {
    fn speak(&self) {
        println!("The {} said {}", self.name(), self.sentence());
    }
}

pub fn inheritance_example() {
    let dog = Dog {};
    let cat = Cat {};
    dog.speak();
    cat.speak();
    println!("type is {}", type_of(dog));

    let human: Box<dyn Human> = Box::new(Person {});
    human.speak();
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
