use scraper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://finance.yahoo.co.jp/stocks/ranking/up/")?.text()?;
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("span._1QEp9BsV").unwrap();
    let elements = document.select(&selector);

    let mut rates = Vec::new();
    elements.for_each(|e| rates.push(e.text().next().unwrap().parse::<f32>().unwrap()));

    println!("{:?}", rates);
    Ok(())
}
