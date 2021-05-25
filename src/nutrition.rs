type Kcal = usize;
type MilligramsPer100Gram = usize;
type Milligrams = usize;

const CARBOHYDRATES_CALORIES_PER_GRAM: Kcal = 4;
const FATS_CALORIES_PER_GRAM: Kcal = 9;
const PROTEINS_CALORIES_PER_GRAM: Kcal = 4;

pub struct Food {
    pub name: String,
    pub carbs: MilligramsPer100Gram,
    pub fats: MilligramsPer100Gram,
    pub prots: MilligramsPer100Gram,
}

impl Food {
    pub fn calculate_calories(&self, weight: Milligrams) -> Kcal {
        ((weight as f64 * self.carbs as f64 / 100_000.0) * CARBOHYDRATES_CALORIES_PER_GRAM as f64
            + (weight as f64 * self.fats as f64 / 100_000.0) * FATS_CALORIES_PER_GRAM as f64
            + (weight as f64 * self.prots as f64 / 100_000.0) * PROTEINS_CALORIES_PER_GRAM as f64)
            as Kcal
    }
}

type Portion = (Food, Milligrams);

pub struct Meal {
    pub foods: Vec<Portion>,
}

impl Meal {
    pub fn calculate_calories(self) -> Kcal {
        self.foods
            .iter()
            .map(|(f, w)| f.calculate_calories(*w))
            .sum()
    }
}

#[cfg(test)]
mod food_tests {
    use super::*;

    #[test]
    fn test_carbs_calories() {
        let food = Food {
            name: "something with only carbs".to_string(),
            carbs: 30_000,
            fats: 0,
            prots: 0,
        };

        assert_eq!(120, food.calculate_calories(100));
    }

    #[test]
    fn test_fats_calories() {
        let food = Food {
            name: "something with only fats".to_string(),
            carbs: 0,
            fats: 20_000,
            prots: 0,
        };

        assert_eq!(180, food.calculate_calories(100));
    }

    #[test]
    fn test_prots_calories() {
        let food = Food {
            name: "something with only prots".to_string(),
            carbs: 0,
            fats: 0,
            prots: 30_000,
        };

        assert_eq!(120, food.calculate_calories(100));
    }

    #[test]
    fn test_food_calories() {
        let peanuts = Food {
            name: "Raw Peanuts".to_string(),
            fats: 49_200,
            carbs: 16_100,
            prots: 25_800,
        };

        assert_eq!(610, peanuts.calculate_calories(100));
    }
}

#[cfg(test)]
mod meal_tests {
    use super::*;

    #[test]
    fn test_meal_calories() {
        let peanuts = Food {
            name: "Peanuts, Raw".to_string(),
            fats: 49_200,
            carbs: 16_100,
            prots: 25_800,
        }; // 122 Kcal

        let scrambled_eggs = Food {
            name: "Eggs, Scrambled".to_string(),
            fats: 5_600,
            carbs: 7_500,
            prots: 13_100,
        }; // 159 Kcal

        let breakfast = Meal {
            foods: vec![(peanuts, 20), (scrambled_eggs, 120)],
        };

        assert_eq!(281, breakfast.calculate_calories());
    }
}
