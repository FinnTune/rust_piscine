macro_rules! hashmap {
    ($( $key:expr => $value:expr ),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

macro_rules! vec_strs {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(String::from($x));
            )*
            temp_vec
        }
    };
}

#[macro_use]
extern crate paste;

macro_rules! define_struct_with_getters {
    ($struct_name:ident { $($field_name:ident : $field_type:ty),* }) => {
        struct $struct_name {
            $($field_name: $field_type),*
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
    };
}

define_struct_with_getters! {
    Person {
        name: String,
        age: u32
    }
}

macro_rules! some_macro {
    ($var_name:ident) => {
        let $var_name = 42;
    };
}

fn main() {
    let map = hashmap!("a" => 1, "b" => 2);
    println!("{:?}", map);

    let v: Vec<String> = vec_strs!("a", "b", "c");
    println!("{:?}", v);

    // Create an instance of Person
    let person = Person { name: "Alice".to_string(), age: 30 };

    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());

    some_macro!(foo);
    println!("{}", foo);
}
