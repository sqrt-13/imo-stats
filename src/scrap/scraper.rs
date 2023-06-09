pub mod scrap;

/// Receive a list of urls and extract each url
pub struct Scraper {
    pub start_url: Url,
}

pub impl Scraper {

    fn handle_parse() {
        let document = Html::parse_document(&text);
let selector = Selector::parse(r#"table > tbody > tr > td > a"#).unwrap();
for title in document.select(&selector) {
    println!("{}", resp.url().to_string());
    println!(
        "{}",
        title
            .value()
            .attr("href")
            .expect("href not found")
            .to_string()
    );
}

    }

}


