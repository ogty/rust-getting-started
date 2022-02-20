struct SVM {
    pub alpha: Vec<f64>,
    pub b: f64,
    pub kernel: Box<dyn Fn(f64, f64) -> f64>,
    pub x: Vec<Vec<f64>>,
    pub y: Vec<f64>,
}


impl SVM {
    pub fn new(x: Vec<Vec<f64>>, y: Vec<f64>, kernel: Box<dyn Fn(f64, f64) -> f64>) -> SVM {
        let alpha = vec![0.0; x.len()];
        SVM {
            alpha,
            b: 0.0,
            kernel,
            x,
            y,
        }
    }


    // kernel function
    pub fn k(&self, x1: &Vec<f64>, x2: &Vec<f64>) -> f64 {
        (self.kernel)(x1, x2)
    }

    pub fn train(&mut self, C: f64, tol: f64, max_iter: usize) {
        let mut iter = 0;
        let mut alpha_changed = 1;
        while iter < max_iter && alpha_changed > 0 {
            alpha_changed = 0;
            for i in 0..self.x.len() {
                let mut y_i = self.y[i];
                let mut alpha_i = self.alpha[i];
                let mut E_i = self.E(i);
                if (y_i * E_i < -tol && alpha_i < C) || (y_i * E_i > tol && alpha_i > 0.0) {
                    let mut j = self.select_j(i, self.x.len());
                    let mut y_j = self.y[j];
                    let mut alpha_j = self.alpha[j];
                    let mut E_j = self.E(j);
                    let mut L = 0.0;
                    let mut H = 0.0;
                    if y_i != y_j {
                        L = max(0.0, alpha_j - alpha_i);
                        H = min(C, C + alpha_j - alpha_i);
                    } else {
                        L = max(0.0, alpha_j + alpha_i - C);
                        H = min(C, alpha_j + alpha_i);
                    }
                    if L == H {
                        continue;
                    }
                    let eta = 2.0 * self.kernel(self.x[i], self.x[j]) - self.kernel(self.x[i], self.x[i]) - self.kernel(self.x[j], self.x[j]);
                    if eta >= 0.0 {
                        continue;
                    }
                    let alpha_j_new_unc = alpha_j - y_j * (E_i - E_j) / eta;
                    let alpha_j_new = self.clip(alpha_j_new_unc, L, H);
                    if alpha_j_new == alpha_j {
                        continue;
                    }
                    alpha_i += y_i * y_j * (alpha_j - alpha_j_new);
                    self.update_E(i);
                    self.update_E(j);
                    alpha_changed += 1;
                    self.alpha[j] = alpha_j_new;
                }
            }
            iter += 1;
        }
    }

    pub fn clip(&self, alpha_j_new_unc: f64, L: f64, H: f64) -> f64 {
        if alpha_j_new_unc > H {
            return H;
        }
        if alpha_j_new_unc < L {
            return L;
        }
        alpha_j_new_unc
    }

    pub fn update_E(&mut self, i: usize) {
        let mut E_i = self.E(i);
        let alpha_i = self.alpha[i];
        let y_i = self.y[i];
        for j in 0..self.x.len() {
            E_i += self.alpha[j] * self.y[j] * self.kernel(self.x[i], self.x[j]);
        }
        E_i -= self.b;
        self.E[i] = E_i;
    }

    pub fn E(&self, i: usize) -> f64 {
        let mut E_i = 0.0;
        for j in 0..self.x.len() {
            E_i += self.alpha[j] * self.y[j] * self.kernel(self.x[i], self.x[j]);
        }
        E_i -= self.b;
        E_i
    }

    pub fn select_j(&self, i: usize, m: usize) -> usize {
        let mut j = i;
        while j == i {
            j = (rand::random::<f64>() * (m as f64)).floor() as usize;
        }
        j
    }

    pub fn predict(&self, x: Vec<f64>) -> f64 {
        let mut b = 0.0;
        let mut w = vec![0.0; self.x[0].len()];
        for i in 0..self.x.len() {
            let alpha_i = self.alpha[i];
            let y_i = self.y[i];
            let k = self.kernel(self.x[i], x);
            for j in 0..self.x[0].len() {
                w[j] += alpha_i * y_i * k;
            }
            b += alpha_i * y_i * k;
        }
        b -= self.b;
        let mut y_predict = 0.0;
        for i in 0..w.len() {
            y_predict += w[i] * x[i];
        }
        y_predict += b;
        if y_predict > 0.0 {
            1.0
        } else {
            -1.0
        }
    }
}

pub fn main() {
    let x = vec![vec![1.0, 1.0], vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, 0.0]];
    let y = vec![1.0, 1.0, -1.0, -1.0];
    let mut svm = SVM::new(x, y, Box::new(|x, y| x * y));
    svm.train(0.1, 0.001, 1000);
    println!("{:?}", svm.predict(vec![1.0, 1.0]));
}
