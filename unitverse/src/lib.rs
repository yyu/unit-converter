mod converter {
    pub mod temperature {
        pub fn c2f(t: f64) -> f64 {
            t * 9.0 / 5.0 + 32.0
        }
        pub fn f2c(t: f64) -> f64 {
            (t - 32.0) * 5.0 / 9.0
        }
    }
}

mod representer {
    pub fn c(t: f64) -> String {
        format!("{}℃", t)
    }
    pub fn f(t: f64) -> String {
        format!("{}℉", t)
    }
}

pub fn celsius_to_farenheit(t: f64) -> f64 {
    crate::converter::temperature::c2f(t)
}

pub fn farenheit_to_celsius(t: f64) -> f64 {
    crate::converter::temperature::f2c(t)
}

pub fn print_celsius(t: f64) {
    let repr = representer::c(t);
    print!("{}", repr);
}

pub fn print_farenheit(t: f64) {
    let repr = representer::f(t);
    print!("{}", repr);
}

#[cfg(test)]
mod tests {
    use crate::converter::temperature;

    #[test]
    fn c2f_works() {
        assert_eq!(temperature::c2f(0.0), 32.0);
        assert_eq!(temperature::c2f(100.0), 212.0);
    }

    #[test]
    fn f2c_works() {
        assert_eq!(temperature::f2c(32.0), 0.0);
        assert_eq!(temperature::f2c(212.0), 100.0);
    }
}
