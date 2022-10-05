use scraper::Html;
use scraper::Selector;
use serde::Serialize;

#[cfg(test)]
mod tests;

#[derive(Debug, Default, Serialize)]
pub struct DiningHall {
    breakfast: Option<Vec<String>>,
    lunch: Option<Vec<String>>,
    dinner: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize)]
pub struct OutRes {
    pub lothian: DiningHall,
    pub glasgow: DiningHall,
}

pub enum Meal {
    Breakfast,
    Lunch,
    Dinner,
}

#[derive(Debug, Clone, Copy)]
pub enum HallType {
    Lothian,
    Glasgow,
}

pub struct HallRequest {
    pub hall: HallType,
    pub url: &'static str,
}

pub const HALL_REQUESTS: [HallRequest; 2] = [
    HallRequest 
    { 
        hall: HallType::Lothian, 
        url: "https://foodpro.ucr.edu/foodpro/shortmenu.asp?sName=University+of+California%2C+Riverside+Dining+Services&locationNum=02&locationName=Lothian+Residential+Restaurant&naFlag=1"}, 
    HallRequest 
    { 
        hall: HallType::Glasgow, 
        url: "https://foodpro.ucr.edu/foodpro/shortmenu.asp?sName=University%20of%20California%2C%20Riverside%20Dining%20Services&locationNum=03&locationName=Glasgow&naFlag=1"
    }
];

impl Meal {
    fn from_str(text: &str) -> Option<Self> {
        match text {
            "Breakfast" => Some(Meal::Breakfast),
            "Lunch" => Some(Meal::Lunch),
            "Dinner" => Some(Meal::Dinner),
            _ => None,
        }
    }
}

/// Takes in an html string and returns [DiningHall], which is a struct that
/// holds a vector of dishes for all meals of the day. HTML Parsing & Querying
/// done with `scraper.rs`
/// 
/// The basic algorithm is as follows:
/// 
/// 1. query for the three menus which hold the meals of the day
/// 2. for each menu
///     - get the name of the menu (i.e. breakfast, lunch, dinner) and store it in [Meal]
///     - get all dishes and store them in a `Vec<String>`
pub fn parse(html: &str, hall_type: HallType) -> DiningHall {
    let document = Html::parse_document(html);

    let mut dining_hall = DiningHall {
        breakfast: None,
        lunch: None,
        dinner: None,
    };

    let meals = Selector::parse(
        match hall_type {
            HallType::Glasgow => {
        "div.pagewrapper:nth-child(5) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(2) > table:nth-child(4) > tbody:nth-child(1) > tr:nth-child(1) > *"
            }
            HallType::Lothian => {
        "div.pagewrapper:nth-child(5) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(2) > table:nth-child(5) > tbody:nth-child(1) > tr:nth-child(1) > *"
            }
        }
    ).unwrap();
    let meal_type = Selector::parse(".shortmenumeals").unwrap();
    let item = Selector::parse("[name='Recipe_Desc']").unwrap();
    for meal in document.select(&meals) {
        if let Some(meal_type) = meal.select(&meal_type).next() {
            let meal_type = Meal::from_str(&meal_type.text().collect::<String>());
            let items: Vec<String> = meal
                .select(&item)
                .map(|i| i.text().collect::<String>())
                .collect();

            if let Some(m) = meal_type {
                match m {
                    Meal::Breakfast => dining_hall.breakfast = Some(items),
                    Meal::Lunch => dining_hall.lunch = Some(items),
                    Meal::Dinner => dining_hall.dinner = Some(items),
                }
            }
        }
    }
    dining_hall
}

