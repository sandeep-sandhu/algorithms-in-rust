// File: visualize.rs
// Purpose: Visualize data structures


// utility function
pub fn print_vector<T: std::fmt::Display>(vector: &Vec<T>){
    for i in 0..vector.len() {
        if i==0{
            print!("Vector: {}", vector[i]);
        }
        else if i==(vector.len()-1) {
            print!(", {}\n", vector[i]);
        }
        else {
            print!(", {}", vector[i]);
        }
    }
    print!("\n");
}
