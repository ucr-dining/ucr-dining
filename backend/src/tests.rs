use crate::{parse, HallType};

#[test]
/// makes sure that we can parse out the html of one of the dining halls into a collection of breakfast, lunch, and dinner
fn test_lothian() {
    let html = std::fs::read_to_string("test_resources/lothian.html").unwrap();
    let dining_hall = parse(&html, HallType::Lothian);
    dbg!(dining_hall);
}

#[test]
/// makes sure that we can parse out the html of one of the dining halls into a collection of breakfast, lunch, and dinner
fn test_glasgow() {
    let html = std::fs::read_to_string("test_resources/glasgow.html").unwrap();
    let dining_hall = parse(&html, HallType::Glasgow);
    dbg!(dining_hall);
}