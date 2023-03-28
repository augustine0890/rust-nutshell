#![allow(dead_code)]

fn average(a: f32, b: f32, c: f32) -> f32 {
    (a + b + c)/3 as f32
}

fn square(x: i32) -> (i32, i32) {
    (x, x * x)
}

// Convert the celsius to fahrenheit
fn convert_temp(cel_temp: f64) -> f64 {
    (1.8 * cel_temp) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        let a = 13 as f32;
        let b = 2.3;
        let c: f32 = 120.0;

        let mut avg = average(a, b, c);
        approx::assert_relative_eq!(avg, 45.1);

        avg = (avg * 100.0).round() / 100.0;
        assert_eq!(avg, 45.1);
    }

    #[test]
    fn test_square() {
        let x = 3;
        assert_eq!(square(x), (3, 9));
    }

    #[test]
    fn test_convert_temp() {
        let celsius_temp = 23.0;
        let fahrenheit_temp = convert_temp(celsius_temp);
        assert_eq!(fahrenheit_temp, 73.4);
    }
}
