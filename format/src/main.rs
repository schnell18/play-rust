use std::collections::HashMap;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
struct Complex {r: f64, i: f64}

fn main() {
    let mut map = HashMap::new();
    map.insert("Portland", (45.5237606, -122.6819273));
    map.insert("Taipei", (25.0375167, 121.5637));
    println!("ordinary print {:?}", map);
    println!("pretty print {:#?}", map);

    // struct
    let third = Complex{ r: -0.5, i: f64::sqrt(0.75)};
   println!("{:?}", third);

    // pointer
    let original = Rc::new("mazurka".to_string());
    let cloned = original.clone();
    let impostor = Rc::new("mazurka".to_string());
   
   println!("text: {} {} {}", original, cloned, impostor);
   println!("pointer: {:p} {:p} {:p}", original, cloned, impostor);
}

#[test]
fn test_positional_mixed() {
    assert_eq!(
        format!("{description:.<25}{quantity:2} @ {price:5.2}", price=3.25, quantity=3, description="Maple Turmeric Latte"),
        "Maple Turmeric Latte..... 3 @  3.25"
    );
    assert_eq!(
        format!("{mode} {2} {} {}", "people", "eater", "purple", mode="flying"),
        "flying purple people eater"
    );
}