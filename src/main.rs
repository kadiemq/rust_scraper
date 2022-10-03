use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Property {
    link: String,
    price: String,
    currency: String,
    location: String,
    property_type: String,
    title: String,
    num_bedrooms: String,
    num_bathrooms: String,
    area: String 
}

impl Property {
    fn new(link: String, price: String, currency: String, location: String, property_type: String, title: String, num_bedrooms: String, num_bathrooms: String, area: String) -> Property {
        Property { link, price, currency, location, property_type, title, num_bedrooms, num_bathrooms, area }
    }

    fn save_to_json(self) -> Result<(), std::io::Error> {
        let path = String::from("outputs/") + &self.title.to_string() + &String::from(".json");
        std::fs::write(
            path,
            serde_json::to_string_pretty(&self).unwrap(),
        )
    }
}

fn main() {
    let response = reqwest::blocking::get("https://www.bayut.com/for-sale/apartments/dubai/").unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let article_selector = scraper::Selector::parse("li.ef447dde>article").unwrap();

    /**
    * 
     * @dev Selectors using "aria-label"
     *
    */
    // let link_selector = scraper::Selector::parse("[aria-label=\"Listing link\"]").unwrap();
    // let price_selector = scraper::Selector::parse("[aria-label=\"Price\"]").unwrap();
    // let currency_selector = scraper::Selector::parse("[aria-label=\"Currency\"]").unwrap();
    // let location_selector = scraper::Selector::parse("[aria-label=\"Location\"]").unwrap();
    // let type_selector = scraper::Selector::parse("[aria-label=\"Type\"]").unwrap();
    // let title_selector = scraper::Selector::parse("[aria-label=\"Title\"]").unwrap();
    // let num_bedrooms_selector = scraper::Selector::parse("[aria-label=\"Beds\"]").unwrap();
    // let num_bathrooms_selector = scraper::Selector::parse("[aria-label=\"Baths\"]").unwrap();
    // let area_selector = scraper::Selector::parse("[aria-label=\"Area\"]>span").unwrap();

    let link_selector = scraper::Selector::parse("a._287661cb").unwrap();
    let location_selector = scraper::Selector::parse("div._7afabd84").unwrap();
    let price_selector = scraper::Selector::parse("span.f343d9ce").unwrap();
    let currency_selector = scraper::Selector::parse("span.c2cc9762").unwrap();
    let type_selector = scraper::Selector::parse("div._9a4e3964").unwrap();
    let title_selector = scraper::Selector::parse("h2._7f17f34f").unwrap();
    let num_bedrooms_selector = scraper::Selector::parse("span.b6a29bc0").unwrap();
    let num_bathrooms_selector = scraper::Selector::parse("span.b6a29bc0").unwrap();
    let area_selector = scraper::Selector::parse("span.b6a29bc0").unwrap();

    let properties = document.select(&article_selector).map(|x| x.inner_html());

    for item in properties {
        let article_html = scraper::Html::parse_document(&item);

        let base_url = String::from("https://www.bayut.com");
        let property_uri = article_html.select(&link_selector).next().unwrap().value().attr("href").unwrap_or
        ("Error").to_string();
        let property_url = base_url + &property_uri;
        let property_price = article_html.select(&price_selector).next().unwrap().inner_html();
        let property_currency = article_html.select(&currency_selector).next().unwrap().inner_html();
        let property_location = article_html.select(&location_selector).next().unwrap().inner_html();
        let property_type = article_html.select(&type_selector).next().unwrap().inner_html();
        let property_title = article_html.select(&title_selector).next().unwrap().inner_html();
        let property_num_bedrooms = article_html.select(&num_bedrooms_selector).next().unwrap().inner_html();
        let property_num_bathrooms = article_html.select(&num_bathrooms_selector).next().unwrap().inner_html();
        let property_area = article_html.select(&area_selector).next().unwrap().inner_html();

        let property = Property::new(property_url, property_price, property_currency, property_location, property_type, property_title, property_num_bedrooms, property_num_bathrooms, property_area);

        let serialized_property = serde_json::to_string(&property).unwrap();

        println!("{}", serialized_property);
        property.save_to_json();
    }
}
