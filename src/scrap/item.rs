pub mod item;

use scraper::{Html, Selector};
use isocountry::CountryCode;

#[derive(Debug)]
pub enum Type {
    Rent,
    Sell
}

#[derive(Debug)]
pub struct Coords {
    pub lat: String,
    pub long: String, 
}

#[derive(Debug)]
pub struct Location {
    pub country: CountryCode,
    pub city: String,
    pub address: Option<String>,
    pub coords: Option<Coords>,
}

#[derive(Debug)]
pub struct Attributes {
    roomCount: u16,
    balconyCount: u16,
    toiletCount: u16,
    usableArea: u64,
    floor: u16,
    year: u16,
}

#[derive(Debug)]
pub struct BuildingFacilities{

}

#[derive(Debug)]
pub struct Item {
    pub r#type: Type,
    pub location: Location,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub attributes: Attributes,
    pub originalUrl: Url,
    pub buildingFacilities: BuildingFacilities,
}
