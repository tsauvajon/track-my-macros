type Kcal = f64;
type MilligramsPer100Gram = usize;
type Milligrams = usize;
type Kilograms = usize;
type Centimeters = usize;
type YearsOld = usize;

const CARBOHYDRATES_CALORIES_PER_GRAM: Kcal = 4.0;
const FATS_CALORIES_PER_GRAM: Kcal = 9.0;
const PROTEINS_CALORIES_PER_GRAM: Kcal = 4.0;

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

        assert_eq!(120, food.calculate_calories(100) as u64);
    }

    #[test]
    fn test_fats_calories() {
        let food = Food {
            name: "something with only fats".to_string(),
            carbs: 0,
            fats: 20_000,
            prots: 0,
        };

        assert_eq!(180, food.calculate_calories(100) as u64);
    }

    #[test]
    fn test_prots_calories() {
        let food = Food {
            name: "something with only prots".to_string(),
            carbs: 0,
            fats: 0,
            prots: 30_000,
        };

        assert_eq!(120, food.calculate_calories(100) as u64);
    }

    #[test]
    fn test_food_calories() {
        let peanuts = Food {
            name: "Raw Peanuts".to_string(),
            fats: 49_200,
            carbs: 16_100,
            prots: 25_800,
        };

        assert_eq!(610, peanuts.calculate_calories(100) as u64);
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

        assert_eq!(281.44, breakfast.calculate_calories());
    }
}

pub struct Human {
    pub weight: Kilograms,
    pub height: Centimeters,
    pub age: YearsOld,
    pub sex: Sex,
}

pub enum Sex {
    Male,
    Female,
}

pub enum ActivityRate {
    Sedentary,        // little to no exercise + work a desk job
    LightlyActive,    // light exercise 1-3 days / week
    ModeratelyActive, // moderate exercise 3-5 days / week
    VeryActive,       // heavy exercise 6-7 days / week
    ExtremelyActive,  // strenuous training 2x / day
}

impl Human {
    /// Basal Metabolic Rate is the energy required to keep your body
    /// functioning at rest.
    ///
    /// Using the Mifflin-St Jeor formula: https://pubmed.ncbi.nlm.nih.gov/2305711/.
    /// Some factors are influent but not taken into account here.
    /// Uncontrollable factors:
    ///   - race (the formula suits Caucasian adults)
    ///   - resting metabolic rate (RMR)
    ///   - genetics
    ///  Controllable factors:
    ///   - stimulants usage
    ///   - lean body mass
    ///   - sleep
    ///   - starvation
    ///   - ...
    ///
    /// See https://blog.nasm.org/nutrition/resting-metabolic-rate-how-to-calculate-and-improve-yours
    fn bmr(&self) -> f64 {
        (10.0 * self.weight as f64)
            + (6.25 * self.height as f64)
            + (5.0 * self.age as f64)
            + match self.sex {
                Sex::Male => 5.0,
                Sex::Female => -161.0,
            }
    }

    /// Total Daily Energy Expenditure
    fn tdee(&self, activity_rate: ActivityRate) -> Kcal {
        self.bmr()
            * match activity_rate {
                ActivityRate::Sedentary => 1.2,
                ActivityRate::LightlyActive => 1.375,
                ActivityRate::ModeratelyActive => 1.55,
                ActivityRate::VeryActive => 1.725,
                ActivityRate::ExtremelyActive => 1.9,
            }
    }
}

#[cfg(test)]
mod human_tests {
    use super::*;

    fn kevin() -> Human {
        Human {
            weight: 70,
            height: 170,
            age: 24,
            sex: Sex::Male,
        }
    }

    fn karen() -> Human {
        Human {
            weight: 70,
            height: 170,
            age: 24,
            sex: Sex::Female,
        }
    }

    fn maurice() -> Human {
        Human {
            weight: 93,
            height: 185,
            age: 56,
            sex: Sex::Male,
        }
    }

    #[test]
    fn test_bmr() {
        assert_eq!(1887, kevin().bmr() as u64, "kevin");
        assert_eq!(1721, karen().bmr() as u64, "karen");
        assert_eq!(2371, maurice().bmr() as u64, "maurice");
    }

    #[test]
    fn test_tdee() {
        assert_eq!(
            2265,
            kevin().tdee(ActivityRate::Sedentary) as u64,
            "Sedentary"
        );

        assert_eq!(
            2595,
            kevin().tdee(ActivityRate::LightlyActive) as u64,
            "LightlyActive"
        );

        assert_eq!(
            2925,
            kevin().tdee(ActivityRate::ModeratelyActive) as u64,
            "ModeratelyActive"
        );

        assert_eq!(
            3255,
            kevin().tdee(ActivityRate::VeryActive) as u64,
            "VeryActive"
        );

        assert_eq!(
            3586,
            kevin().tdee(ActivityRate::ExtremelyActive) as u64,
            "ExtremelyActive"
        );
    }
}
