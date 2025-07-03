fn f(t : f64 , y : f64 ) -> f64 {
    t.cos() - y
}


fn main() {
   let t = 0.0 ;
   let y = 1.0 ;

    println!("f(t = {t} , y ={y}) = {}" , f(t,y)) ;
}
