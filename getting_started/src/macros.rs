macro_rules! myvec {
    ($elem:expr, $n:expr) => {
        ::std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ),* ) => {
        <[_]>::into_vec(Box::new([ $( $x ),* ]))
    };
    ( $( $x:expr ), +, ) => {
        myvec![ $( $x ),* ]
    };
}


pub fn main() {
    let v = myvec![1, 2, 3];
    println!("{:?}", v);
}
