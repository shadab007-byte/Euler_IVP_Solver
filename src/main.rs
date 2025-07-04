use std::fs::File;
use std::io::{Write, BufWriter};



fn f(t : f64 , y : f64 ) -> f64 {
    t.cos() - y
}

fn euler_solver(f : fn(f64,f64) -> f64 , t0 : f64 , y0: f64 , t_end : f64 , n : usize) -> (Vec<f64>, Vec<f64>){

    let h = (t_end - t0) / n as f64 ;
    let mut t_values = vec![t0];
    let mut y_values = vec![y0];

    let mut t = t0;
    let mut y = y0;

    for _ in 0..n {
        y = y + h*f(t,y);
        t = t + h;
        t_values.push(t);
        y_values.push(y);

    }

    (t_values , y_values)

}


fn save_to_csv(t_vals: &Vec<f64>, y_vals: &Vec<f64>, filename: &str) {
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "t,y").unwrap(); // header
    for (t, y) in t_vals.iter().zip(y_vals.iter()) {
        writeln!(writer, "{},{}", t, y).unwrap();
    }
}

fn main() {
   let t0 = 0.0;
   let y0 = 1.0;
   let t_end = 5.0;
   let n = 20;
   
   let (t_vals, y_vals) = euler_solver(f, t0, y0, t_end, n);

    for (t, y) in t_vals.iter().zip(y_vals.iter()) {
        println!("t = {:.2}, y = {:.5}", t, y);
    }

     // Export to CSV
    save_to_csv(&t_vals, &y_vals, "rust_results.csv");
    println!("Results saved to rust_results.csv");
    
}
