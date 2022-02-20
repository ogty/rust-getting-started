// decistion tree struct
struct DecisionTree {
    root: Node,
}


// decistion tree methods
impl DecisionTree {
    // constructor
    fn new(x: Vec<f64>, y: Vec<f64>) -> DecisionTree {
        let root = Node::new(x, y);
        DecisionTree { root }
    }

    // predict
    fn predict(&self, x: f64) -> f64 {
        self.root.predict(x)
    }
}


// node struct
struct Node {
    x: f64,
    y: f64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}


// node methods
impl Node {
    // constructor
    fn new(x: Vec<f64>, y: Vec<f64>) -> Node {
        let mut x_sorted = x.clone();
        x_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let x_median = x_sorted[x_sorted.len() / 2];

        let mut y_sorted = y.clone();
        y_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let y_median = y_sorted[y_sorted.len() / 2];

        Node {
            x: x_median,
            y: y_median,
            left: None,
            right: None,
        }
    }

    // predict
    fn predict(&self, x: f64) -> f64 {
        if x < self.x {
            if self.left.is_some() {
                self.left.as_ref().unwrap().predict(x)
            } else {
                self.y
            }
        } else {
            if self.right.is_some() {
                self.right.as_ref().unwrap().predict(x)
            } else {
                self.y
            }
        }
    }
}


// main
pub fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    let tree = DecisionTree::new(x, y);
    println!("{}", tree.predict(3.0));
}
