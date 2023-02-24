use std::env;

extern crate reqwest;

struct Teams {
    team_name: String,
    team_link: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let response = reqwest::blocking::get(
        &args[1],
    )
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response); 

    let table_selector = scraper::Selector::parse(".table.is-middle-aligned").unwrap();
    let table_anchor_selector = scraper::Selector::parse("td a").unwrap();
    let table_span_selector = scraper::Selector::parse("td span").unwrap();

    for element in document.select(&table_selector){
        let anchor_element = element.select(&table_anchor_selector).next().expect("Failed to find anchor elmeent");
        let anchor_element_href = anchor_element.value().attr("href").expect("Failed to find href attribute");
        println!("{:?}",anchor_element_href);
    }

}
