// linear regression struct
struct LinearRegression {
    slope: f64,
    intercept: f64,
}


// linear regression methods
impl LinearRegression {
    // constructor
    fn new(x: Vec<f64>, y: Vec<f64>) -> LinearRegression {
        let slope = LinearRegression::slope(&x, &y);
        let intercept = LinearRegression::intercept(&x, &y, slope);
        LinearRegression { slope, intercept }
    }

    // slope
    fn slope(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;
        let mut n = x.len() as f64;

        for i in 0..x.len() {
            sum_x += x[i];
            sum_y += y[i];
            sum_xy += x[i] * y[i];
            sum_x2 += x[i] * x[i];
        }

        (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x)
    }

    // intercept
    fn intercept(x: &Vec<f64>, y: &Vec<f64>, slope: f64) -> f64 {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut n = x.len() as f64;

        for i in 0..x.len() {
            sum_x += x[i];
            sum_y += y[i];
        }

        (sum_y - slope * sum_x) / n
    }

    // predict
    fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}


// main
pub fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let y = vec![2.0, 3.0, 4.0, 5.0];

    let lr = LinearRegression::new(x, y);

    println!("Slope: {}", lr.slope);
    println!("Intercept: {}", lr.intercept);
    println!("Prediction for x = 3.5: {}", lr.predict(3.5));
}
