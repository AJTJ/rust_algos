fn main() {
    // using traits only
    pub trait Speak {
        fn speak(&self);
    }

    struct Cat {
        fur: String,
    }

    impl Speak for Cat {
        fn speak(&self) {
            println!("Meow");
        }
    }

    let my_cat = Cat {
        fur: "red".to_string(),
    };

    my_cat.speak();

    // generics
    struct Vehicle<Kind: ?Sized> {
        color: String,
        kind: Kind,
    }

    trait IsArmed {
        fn is_armed(vehicle: &Vehicle<Self>) {}
    }

    impl<K: IsArmed> Vehicle<K> {
        // stuff that all vehicles can do
        fn is_armed(&self) {
            K::is_armed(self)
        }
    }

    struct Tank {}

    impl IsArmed for Tank {
        fn is_armed(vehicle: &Vehicle<Self>) {
            if vehicle.color == "Green" {
                println!("Green tanks are fast!")
            } else {
                println!("Any tank but Green tanks are still armed, but slow.")
            }
        }
    }

    impl Vehicle<Tank> {
        // stuff that only tanks can do
        fn shoot(&self) {
            println!("bang")
        }
    }

    let my_tank = Vehicle {
        kind: Tank {},
        color: "Green".to_string(),
    };

    my_tank.is_armed();
    my_tank.shoot();

    struct SexyCar {};
    impl Vehicle<SexyCar> {
        fn act_sexy() {
            println!("Oooo so sexy")
        }
    }

    impl IsArmed for SexyCar {
        fn is_armed(vehicle: &Vehicle<Self>) {
            println!("no guns in this sexy {} car", vehicle.color);
        }
    }

    let my_sexy_car = Vehicle {
        kind: SexyCar {},
        color: "Yellow".to_string(),
    };

    my_sexy_car.is_armed();

    // another method

    pub trait Building {
        fn footage(&self) -> i32;
    }

    struct House {
        doors: i32,
    }

    struct BigHouse {
        house_properties: House,
    }

    impl Building for House {
        fn footage(&self) -> i32 {
            200
        }
    }

    impl Building for BigHouse {
        fn footage(&self) -> i32 {
            2000
        }
    }

    let house_properties = House { doors: 32 };

    let my_big_house = BigHouse { house_properties };

    println!(
        "{}, {}",
        my_big_house.footage(),
        my_big_house.house_properties.doors
    );
}
