mod sigaa_time;

fn main() {
    let today = sigaa_time::Weekday::new(5).unwrap(); 
    println!("{}", today);
}
