// units
type Kcal = f64;
type MilligramsPer100Gram = usize;
type Milligrams = usize;
type Grams = usize;
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
    pub activity_rate: ActivityRate,
}

pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, Clone)]
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
    fn tdee(&self) -> Kcal {
        self.bmr()
            * match self.activity_rate {
                ActivityRate::Sedentary => 1.2,
                ActivityRate::LightlyActive => 1.375,
                ActivityRate::ModeratelyActive => 1.55,
                ActivityRate::VeryActive => 1.725,
                ActivityRate::ExtremelyActive => 1.9,
            }
    }

    fn suggest_daily_prot_needs(&self) -> Grams {
        (self.weight as f64
            * match self.activity_rate {
                ActivityRate::Sedentary => 1.0,
                ActivityRate::LightlyActive => 1.2,
                ActivityRate::ModeratelyActive => 1.4,
                ActivityRate::VeryActive => 1.8,
                ActivityRate::ExtremelyActive => 2.2,
            }) as Grams
    }

    fn suggest_daily_fat_needs(&self) -> Grams {
        self.weight as Grams
    }

    /// See https://www.nasm.org/resources/calorie-calculator
    pub fn suggest_fixed_goal(&self) -> CalorieIntakeRepartition {
        let prots = self.suggest_daily_prot_needs();
        let fats = self.suggest_daily_fat_needs();

        let carbs_calories_needed = self.tdee()
            - prots as f64 * PROTEINS_CALORIES_PER_GRAM
            - fats as f64 * FATS_CALORIES_PER_GRAM;

        let carbs = (carbs_calories_needed / CARBOHYDRATES_CALORIES_PER_GRAM) as Grams;

        CalorieIntakeRepartition::Fixed { prots, fats, carbs }
    }

    pub fn suggest_percentage_goal(&self) -> CalorieIntakeRepartition {
        match self.suggest_fixed_goal() {
            CalorieIntakeRepartition::Fixed { fats, carbs, prots } => {
                let fats_calories = fats as f64 * FATS_CALORIES_PER_GRAM;
                let prots_calories = prots as f64 * PROTEINS_CALORIES_PER_GRAM;
                let carbs_calories = carbs as f64 * CARBOHYDRATES_CALORIES_PER_GRAM;

                let total = fats_calories + prots_calories + carbs_calories;

                println!(
                    "{} {} {} {}",
                    fats_calories, prots_calories, carbs_calories, total
                );

                let fats = (100.0 * fats_calories / total) as u64;
                let prots = (100.0 * prots_calories / total) as u64;
                let carbs = 100 - fats - prots;

                println!(
                    "{} {} {} {} {}",
                    fats_calories, prots_calories, carbs_calories, fats, prots
                );

                CalorieIntakeRepartition::Percentage { fats, carbs, prots }
            }
            CalorieIntakeRepartition::Percentage { fats, carbs, prots } => {
                CalorieIntakeRepartition::Percentage { fats, carbs, prots }
            }
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
            activity_rate: ActivityRate::ModeratelyActive,
        }
    }

    fn karen() -> Human {
        Human {
            weight: 70,
            height: 170,
            age: 24,
            sex: Sex::Female,
            activity_rate: ActivityRate::Sedentary,
        }
    }

    fn maurice() -> Human {
        Human {
            weight: 93,
            height: 185,
            age: 56,
            sex: Sex::Male,
            activity_rate: ActivityRate::LightlyActive,
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
        let tc = vec![
            (2265, ActivityRate::Sedentary),
            (2595, ActivityRate::LightlyActive),
            (2925, ActivityRate::ModeratelyActive),
            (3255, ActivityRate::VeryActive),
            (3586, ActivityRate::ExtremelyActive),
        ];

        for (want, activity_rate) in tc {
            let mut kevin = kevin();
            kevin.activity_rate = activity_rate.clone();

            assert_eq!(want, kevin.tdee() as u64, "{:?}", activity_rate,);
        }
    }

    #[test]
    fn test_suggest_daily_prot_needs() {
        let tc = vec![
            (70, ActivityRate::Sedentary),
            (84, ActivityRate::LightlyActive),
            (98, ActivityRate::ModeratelyActive),
            (126, ActivityRate::VeryActive),
            (154, ActivityRate::ExtremelyActive),
        ];

        for (want, activity_rate) in tc {
            let mut kevin = kevin();
            kevin.activity_rate = activity_rate.clone();

            assert_eq!(
                want,
                kevin.suggest_daily_prot_needs(),
                "{:?}",
                activity_rate,
            );
        }
    }

    #[test]
    fn test_suggest_daily_fat_needs() {
        assert_eq!(70, kevin().suggest_daily_fat_needs(), "kevin");
        assert_eq!(70, karen().suggest_daily_fat_needs(), "karen");
        assert_eq!(93, maurice().suggest_daily_fat_needs(), "maurice");
    }

    #[test]
    fn test_suggest_fixed_goal() {
        assert_eq!(
            CalorieIntakeRepartition::Fixed {
                fats: 70,   // (70*9) / 2926 ~= 22%
                prots: 98,  // (98*4) / 2926 ~= 13%
                carbs: 475, // (475*4) / 2926 ~= 65%
            },
            kevin().suggest_fixed_goal(),
            "kevin"
        );

        assert_eq!(
            CalorieIntakeRepartition::Fixed {
                fats: 70,
                prots: 70,
                carbs: 288,
            },
            karen().suggest_fixed_goal(),
            "karen"
        );

        assert_eq!(
            CalorieIntakeRepartition::Fixed {
                fats: 93,
                prots: 111,
                carbs: 494,
            },
            maurice().suggest_fixed_goal(),
            "maurice"
        );
    }

    #[test]
    fn test_suggest_percentage_goal() {
        assert_eq!(
            CalorieIntakeRepartition::Percentage {
                fats: 21,
                prots: 13,
                carbs: 66,
            },
            kevin().suggest_percentage_goal(),
            "kevin"
        );

        assert_eq!(
            CalorieIntakeRepartition::Percentage {
                fats: 30,
                prots: 13,
                carbs: 57,
            },
            karen().suggest_percentage_goal(),
            "karen"
        );

        assert_eq!(
            CalorieIntakeRepartition::Percentage {
                fats: 25,
                prots: 13,
                carbs: 62,
            },
            maurice().suggest_percentage_goal(),
            "maurice"
        );
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CalorieIntakeRepartition {
    Fixed {
        fats: Grams,
        prots: Grams,
        carbs: Grams,
    },

    Percentage {
        fats: u64,
        prots: u64,
        carbs: u64,
    },
}
