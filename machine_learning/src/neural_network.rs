// neural network struct
pub struct NeuralNetwork {
    pub input_nodes: usize,
    pub hidden_nodes: usize,
    pub output_nodes: usize,
    pub weights_ih: Matrix,
    pub weights_ho: Matrix,
    pub bias_h: Matrix,
    pub bias_o: Matrix,
}


// neural network methods
impl NeuralNetwork {
    // constructor
    pub fn new(input_nodes: usize, hidden_nodes: usize, output_nodes: usize) -> NeuralNetwork {
        let weights_ih = Matrix::new(hidden_nodes, input_nodes);
        let weights_ho = Matrix::new(output_nodes, hidden_nodes);
        let bias_h = Matrix::new(hidden_nodes, 1);
        let bias_o = Matrix::new(output_nodes, 1);
        NeuralNetwork {
            input_nodes,
            hidden_nodes,
            output_nodes,
            weights_ih,
            weights_ho,
            bias_h,
            bias_o,
        }
    }

    // train
    pub fn train(&mut self, inputs_list: &Vec<Vec<f64>>, targets_list: &Vec<Vec<f64>>, learning_rate: f64, epochs: usize) {
        let mut inputs: Matrix = Matrix::new(inputs_list.len(), self.input_nodes);
        let mut targets: Matrix = Matrix::new(targets_list.len(), self.output_nodes);
        for i in 0..inputs_list.len() {
            for j in 0..inputs_list[i].len() {
                inputs.set(i, j, inputs_list[i][j]);
            }
        }
        for i in 0..targets_list.len() {
            for j in 0..targets_list[i].len() {
                targets.set(i, j, targets_list[i][j]);
            }
        }
        for _ in 0..epochs {
            let outputs = self.feedforward(&inputs);
            let mut errors = Matrix::new(targets.rows, targets.cols);
            for i in 0..targets.rows {
                for j in 0..targets.cols {
                    errors.set(i, j, targets.get(i, j) - outputs.get(i, j));
                }
            }
            let gradients = self.backprop(&errors);
            self.update_weights(&gradients, learning_rate);
        }
    }

    // predict
    pub fn predict(&self, inputs_list: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut inputs: Matrix = Matrix::new(inputs_list.len(), self.input_nodes);
        for i in 0..inputs_list.len() {
            for j in 0..inputs_list[i].len() {
                inputs.set(i, j, inputs_list[i][j]);
            }
        }
        let outputs = self.feedforward(&inputs);
        let mut results: Vec<Vec<f64>> = Vec::new();
        for i in 0..outputs.rows {
            let mut result: Vec<f64> = Vec::new();
            for j in 0..outputs.cols {
                result.push(outputs.get(i, j));
            }
            results.push(result);
        }
        results
    }


    // feedforward
    fn feedforward(&self, inputs: &Matrix) -> Matrix {
        let mut hidden = Matrix::multiply(&self.weights_ih, &inputs);
        hidden.add_matrix(&self.bias_h);
        hidden.map(|x| 1.0 / (1.0 + (-x).exp()));
        let mut outputs = Matrix::multiply(&self.weights_ho, &hidden);
        outputs.add_matrix(&self.bias_o);
        outputs.map(|x| 1.0 / (1.0 + (-x).exp()));
        outputs
    }


    // backprop
    fn backprop(&self, errors: &Matrix) -> (Matrix, Matrix) {
        let mut hidden_errors = Matrix::multiply(&errors, &self.weights_ho.transpose());
        hidden_errors.map(|x| x * (1.0 - x));
        let mut hidden_gradients = Matrix::multiply(&hidden_errors, &self.hidden_outputs());
        let mut output_errors = Matrix::multiply(&self.hidden_outputs().transpose(), &errors);
        output_errors.map(|x| x * (1.0 - x));
        let mut output_gradients = Matrix::multiply(&output_errors, &self.hidden_outputs());
        (hidden_gradients, output_gradients)
    }

    // update_weights
    fn update_weights(&mut self, gradients: &(Matrix, Matrix), learning_rate: f64) {
        let mut hidden_gradients = gradients.0;
        let mut output_gradients = gradients.1;
        hidden_gradients.multiply_constant(learning_rate);
        output_gradients.multiply_constant(learning_rate);
        self.weights_ih.add_matrix(&hidden_gradients);
        self.weights_ho.add_matrix(&output_gradients);
        self.bias_h.add_matrix(&hidden_gradients);
        self.bias_o.add_matrix(&output_gradients);
    }


    // activation_function
    fn activation_function(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }


    // hidden_outputs
    fn hidden_outputs(&self) -> Matrix {
        let mut hidden = Matrix::multiply(&self.weights_ih, &self.inputs());
        hidden.add_matrix(&self.bias_h);
        hidden.map(|x| 1.0 / (1.0 + (-x).exp()));
        hidden
    }


    // inputs
    fn inputs(&self) -> Matrix {
        let mut inputs = Matrix::new(self.input_nodes, 1);
        for i in 0..self.input_nodes {
            inputs.set(i, 0, 1.0);
        }
        inputs
    }


    // save
    pub fn save(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        let mut data = String::new();
        data.push_str(&self.input_nodes.to_string());
        data.push_str("\n");
        data.push_str(&self.hidden_nodes.to_string());
        data.push_str("\n");
        data.push_str(&self.output_nodes.to_string());
        data.push_str("\n");
        data.push_str(&self.weights_ih.to_string());
        data.push_str("\n");
        data.push_str(&self.weights_ho.to_string());
        data.push_str("\n");
        data.push_str(&self.bias_h.to_string());
        data.push_str("\n");
        data.push_str(&self.bias_o.to_string());
        file.write_all(data.as_bytes()).unwrap();
    }


    // load
    pub fn load(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let mut lines = data.lines();
        let input_nodes = lines.next().unwrap().parse::<usize>().unwrap();
        let hidden_nodes = lines.next().unwrap().parse::<usize>().unwrap();
        let output_nodes = lines.next().unwrap().parse::<usize>().unwrap();
        let weights_ih = Matrix::from_string(lines.next().unwrap());
        let weights_ho = Matrix::from_string(lines.next().unwrap());
        let bias_h = Matrix::from_string(lines.next().unwrap());
        let bias_o = Matrix::from_string(lines.next().unwrap());
        NeuralNetwork {
            input_nodes: input_nodes,
            hidden_nodes: hidden_nodes,
            output_nodes: output_nodes,
            weights_ih: weights_ih,
            weights_ho: weights_ho,
            bias_h: bias_h,
            bias_o: bias_o,
        }
    }


    // test
    pub fn test(&self, inputs_list: &Vec<Vec<f64>>, outputs_list: &Vec<Vec<f64>>) -> f64 {
        let mut total_error = 0.0;
        for i in 0..inputs_list.len() {
            let outputs = self.predict(&inputs_list[i]);
            for j in 0..outputs.len() {
                total_error += (outputs[j][0] - outputs_list[i][j]).abs();
            }
        }
        total_error / (inputs_list.len() as f64)
    }


    // predict_list
    pub fn predict_list(&self, inputs_list: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut results: Vec<Vec<f64>> = Vec::new();
        for i in 0..inputs_list.len() {
            results.push(self.predict(&inputs_list[i]));
        }
        results
    }


    // train_list
    pub fn train_list(&mut self, inputs_list: &Vec<Vec<f64>>, outputs_list: &Vec<Vec<f64>>, learning_rate: f64, epochs: usize) {
        for _ in 0..epochs {
            for i in 0..inputs_list.len() {
                let mut inputs = Matrix::new(self.input_nodes, 1);
                for j in 0..self.input_nodes {
                    inputs.set(j, 0, inputs_list[i][j]);
                }
                let mut outputs = Matrix::new(self.output_nodes, 1);
                for j in 0..self.output_nodes {
                    outputs.set(j, 0, outputs_list[i][j]);
                }
                let (hidden_gradients, output_gradients) = self.backprop(&outputs);
                self.update_weights(&(hidden_gradients, output_gradients), learning_rate);
            }
        }
    }
}


// neural network struct and methods
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}


// neural network methods
impl Matrix {
    // constructor
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let mut data: Vec<f64> = Vec::new();
        for _ in 0..rows {
            for _ in 0..cols {
                data.push(0.0);
            }
        }
        Matrix {
            rows,
            cols,
            data,
        }
    }

    // get
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    // set
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    // transpose
    pub fn transpose(&self) -> Matrix {
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self.cols {
            for j in 0..self.rows {
                data.push(self.get(j, i));
            }
        }
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }

    // map
    pub fn map(&self, func: fn(f64) -> f64) -> Matrix {
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                data.push(func(self.get(i, j)));
            }
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    // add
    pub fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self
            .rows {
            for j in 0..self
                .cols {
                data.push(self.get(i, j) + other.get(i, j));
            }
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    // multiply
    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                data.push(sum);
            }
        }
        Matrix {
            rows: self.rows,
            cols: other.cols,
            data,
        }
    }

    // dot
    pub fn dot(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut data: Vec<f64> = Vec::new();
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                data.push(sum);
            }
        }
        Matrix {
            rows: self.rows,
            cols: other.cols,
            data,
        }
    }

    // sum
    pub fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                sum += self.get(i, j);
            }
        }
        sum
    }

    // mean
    pub fn mean(&self) -> f64 {
        self.sum() / (self.rows * self.cols) as f64
    }

    // variance
    pub fn variance(&self) -> f64 {
        let mean = self.mean();
        let mut sum = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                sum += (self.get(i, j) - mean) * (self.get(i, j) - mean);
            }
        }
        sum / (self.rows * self.cols) as f64
    }

    // standard_deviation
    pub fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    // to_string
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                s.push_str(&format!("{:.2} ", self.get(i, j)));
            }
            s.push_str("\n");
        }
        s
    }

    // print
    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}


// activation functions
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}


pub fn sigmoid_derivative(x: f64) -> f64 {
    let s = sigmoid(x);
    s * (1.0 - s)
}


pub fn relu(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        x
    }
}


pub fn relu_derivative(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        1.0
    }
}


pub fn tanh(x: f64) -> f64 {
    x.tanh()
}


pub fn tanh_derivative(x: f64) -> f64 {
    1.0 - x.tanh().powi(2)
}


pub fn softmax(x: &Matrix) -> Matrix {
    let mut max = x.data[0];
    for i in 1..x.data.len() {
        if x.data[i] > max {
            max = x.data[i];
        }
    }
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i].exp() / max);
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn softmax_derivative(x: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] * (1.0 - x.data[i]));
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn softmax_cross_entropy(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += x.data[i] * (y.data[i] - 1.0).abs();
    }
    sum
}


pub fn softmax_cross_entropy_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] * (y.data[i] - 1.0));
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn cross_entropy(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += x.data[i] * (y.data[i] - 1.0).abs();
    }
    sum
}


pub fn cross_entropy_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] * (y.data[i] - 1.0));
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn mean_squared_error(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += (x.data[i] - y.data[i]).powi(2);
    }
    sum / (x.rows * x.cols) as f64
}


pub fn mean_squared_error_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] - y.data[i]);
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn cross_entropy_with_softmax(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += x.data[i] * (y.data[i] - 1.0).abs();
    }
    sum
}


pub fn cross_entropy_with_softmax_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] * (y.data[i] - 1.0));
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn mean_squared_error_with_softmax(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += (x.data[i] - y.data[i]).powi(2);
    }
    sum / (x.rows * x.cols) as f64
}


pub fn mean_squared_error_with_softmax_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] - y.data[i]);
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn softmax_with_cross_entropy(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += x.data[i] * (y.data[i] - 1.0).abs();
    }
    sum
}


pub fn softmax_with_cross_entropy_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] * (y.data[i] - 1.0));
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn softmax_with_mean_squared_error(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += (x.data[i] - y.data[i]).powi(2);
    }
    sum / (x.rows * x.cols) as f64
}


pub fn softmax_with_mean_squared_error_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] - y.data[i]);
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn softmax_with_cross_entropy_with_softmax(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += x.data[i] * (y.data[i] - 1.0).abs();
    }
    sum
}


pub fn softmax_with_cross_entropy_with_softmax_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] * (y.data[i] - 1.0));
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


pub fn softmax_with_mean_squared_error_with_softmax(x: &Matrix, y: &Matrix) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.data.len() {
        sum += (x.data[i] - y.data[i]).powi(2);
    }
    sum / (x.rows * x.cols) as f64
}


pub fn softmax_with_mean_squared_error_with_softmax_derivative(x: &Matrix, y: &Matrix) -> Matrix {
    let mut data: Vec<f64> = Vec::new();
    for i in 0..x.data.len() {
        data.push(x.data[i] - y.data[i]);
    }
    Matrix {
        rows: x.rows,
        cols: x.cols,
        data,
    }
}


// main function
pub fn main() {
    let x = Matrix {
        rows: 2,
        cols: 2,
        data: vec![1.0, 2.0, 3.0, 4.0],
    };
    let y = Matrix {
        rows: 2,
        cols: 2,
        data: vec![2.0, 4.0, 6.0, 8.0],
    };
    println!("{}", mean_squared_error(&x, &y));
    println!("{}", mean_squared_error_derivative(&x, &y));
    println!("{}", cross_entropy_with_softmax(&x, &y));
    println!("{}", cross_entropy_with_softmax_derivative(&x, &y));
    println!("{}", mean_squared_error_with_softmax(&x, &y));
    println!("{}", mean_squared_error_with_softmax_derivative(&x, &y));
    println!("{}", softmax_with_cross_entropy(&x, &y));
    println!("{}", softmax_with_cross_entropy_derivative(&x, &y));
    println!("{}", softmax_with_mean_squared_error(&x, &y));
    println!("{}", softmax_with_mean_squared_error_derivative(&x, &y));
    println!("{}", softmax_with_cross_entropy_with_softmax(&x, &y));
    println!("{}", softmax_with_cross_entropy_with_softmax_derivative(&x, &y));
    println!("{}", softmax_with_mean_squared_error_with_softmax(&x, &y));
    println!("{}", softmax_with_mean_squared_error_with_softmax_derivative(&x, &y));
}
