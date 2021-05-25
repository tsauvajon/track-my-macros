/*!
track-my-macros is an application that can track a detail calorie intake, by
allowing users to manually input their macronutrient intake.
*/

#![deny(
    warnings,
    missing_doc_code_examples,
    missing_docs,
    clippy::all,
    clippy::cargo
)]

mod nutrition;

use nutrition::{Food, Meal};

fn main() {
    println!("Hello, world!");
    let peanuts = Food {
        name: "Peanuts, Raw".to_string(),
        fats: 49_200,
        carbs: 16_100,
        prots: 25_800,
    };

    let scrambled_eggs = Food {
        name: "Eggs, Scrambled".to_string(),
        fats: 5_600,
        carbs: 7_500,
        prots: 13_100,
    };

    let breakfast = Meal {
        foods: vec![(peanuts, 20), (scrambled_eggs, 120)],
    };

    breakfast.calculate_calories();
}
