#[macro_use]
extern crate paste;


macro_rules! define_struct_and_getters {
    ($struct_name:ident {
        $(
        $field_name:ident : $field_type:ty
        ),*
    }) => {
        struct $struct_name {
            $(
            $field_name: $field_type
            ),*
        }

        impl $struct_name {
            $(
            paste! {
                fn [<get_ $field_name>](&self) -> &$field_type {
                    &self.$field_name
                }
            }
            )*
        }
    }
}

define_struct_and_getters! {
    Person {
        name: String,
        age: u32
    }
}


fn main() {
    let person = Person {
        name: String::from("John"),
        age: 42
    };

    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());
}
