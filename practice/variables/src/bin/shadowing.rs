fn main() {
    let x = 5;           // x #1 = 5
    let x = x + 1;       // x #2 = x #1 + 1 → 6, hides #1
    {
        let x = x * 2;   // x #3 = x #2 * 2 → 12, hides #2 inside { }
        println!("inner x is {x}");

    }                         // #3 is gone, x means #2 again → 6
    println!("outer x is {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is {spaces}");
}
