use std::ffi::OsStr;
use std::path::Path;
use std::collections::BTreeSet;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    test_flat_map();
}

#[test]
fn test_vector_iter() {
    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    let v1 = match iterator.next() {
        Some(&i) => i,
        _ => 0
    };
    assert_eq!(v1, 4);
    //assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);
    assert_eq!(v[0], 4);
    assert_eq!(v[1], 20);
}

#[test]
fn test_path_iter() {
    let path = Path::new("c:/Users/JimB/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("c:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("JimB")));
    assert_eq!(iterator.next(), Some(OsStr::new("Downloads")));
    assert_eq!(iterator.next(), Some(OsStr::new("Fedora.iso")));
}

#[test]
fn test_btreeset_iter() {
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the sky With Diamonds".to_string());
    favorites.insert("Liebestraume No. 3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebestraume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the sky With Diamonds".to_string()));
    assert_eq!(it.next(), None);
}

//#[test]
fn test_flat_map() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("US", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("Dutch", vec!["Amsterdam", "Utrecht"]);

    // for &city in major_cities.keys().flat_map(|country| &major_cities[country]) {
    for city in major_cities.keys().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }

    for (i, city) in major_cities.keys().flat_map(|country| &major_cities[country]).enumerate() {
        println!("{}: {}", i+1, city);
    }
}
