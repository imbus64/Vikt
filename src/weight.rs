use crate::weightlog;
use chrono::{DateTime, Local};
use core::fmt;

pub struct WeightT {
    pub kilo: f32,
    pub time: chrono::DateTime<Local>,
}

impl WeightT {
    pub(crate) fn new(weight: f32, time: Option<DateTime<Local>>) -> WeightT {
        return WeightT {
            kilo: weight,
            time: time.unwrap_or(Local::now()),
        };
    }

    // Returns number of hours since creation
    pub fn age(&self) -> f32 { return (Local::now() - self.time).num_hours() as f32 / 24f32; }

    // Calculates 'dumb' BMI. Rounds to two decimals (No string operations)
    // This one will go unused until we have a proper config file
    pub fn bmi(&self, height_cm: f32) -> f32 {
        let bmi_f = self.kilo / (height_cm / 100.0).powi(2);
        let bmi_i = (bmi_f * 100.0) as i32;
        bmi_i as f32 / 100f32
    }
}

impl fmt::Display for WeightT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formattning_string = format!("{} {}", weightlog::DATE_FMT, weightlog::TIME_FMT);
        let dt_string = self.time.format(&formattning_string).to_string();
        let daysago = format!("({:.1} days ago)", self.age());
        write!(f, "{} {} {} kg", dt_string, daysago, self.kilo,)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_weight_t() {
        let w_none = WeightT::new(100 as f32, None);
        let w_some = WeightT::new(100 as f32, Some(Local::now()));
        assert!(w_none.age() < f32::MAX && w_none.age() >= 0.0 as f32);
        assert!(w_some.age() < f32::MAX && w_none.age() >= 0.0 as f32);
    }

    #[test]
    fn dumb_bmi_calculation() {
        let w = WeightT::new(100.0, None);
        assert_eq!(w.bmi(100.0), 100.0);

        // This test case depends on the two decimal rounding in bmi().
        assert_eq!(w.bmi(180.0), 30.86);
    }
}
