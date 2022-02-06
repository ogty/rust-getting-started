use scraper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://github.com/ogty/uiux/")?.text()?;
    let document = scraper::Html::parse_document(&body);
    
    let selector = scraper::Selector::parse("li").unwrap();
    let elements = document.select(&selector);
    elements.for_each(|e| println!("{}", e.text().next().unwrap()));

    Ok(())
}