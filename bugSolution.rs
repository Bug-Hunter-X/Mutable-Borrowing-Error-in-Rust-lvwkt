fn main() {
    let mut x = 5;
    { //Creating a new scope
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 20; 
} 