extern crate json; // Assume the 'json' crate is being used for JSON manipulation

#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub calories: [String; 2], // ["value_in_kJ", "value_in_kcal"]
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_calories = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;

    for food in foods.iter() {
        let kcal: f64 = food.calories[1]
            .replace("kcal", "")
            .parse()
            .expect("Failed to parse calories");

        total_calories += kcal * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
    }

    // Round to two decimal places, or one if it ends in a zero
    total_calories = (total_calories * 100.0).round() / 100.0;
    total_fats = (total_fats * 100.0).round() / 100.0;
    total_carbs = (total_carbs * 100.0).round() / 100.0;
    total_proteins = (total_proteins * 100.0).round() / 100.0;

    json::object! {
        "cals" => total_calories,
        "carbs" => total_carbs,
        "proteins" => total_proteins,
        "fats" => total_fats
    }
}