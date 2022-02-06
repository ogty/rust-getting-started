use core::cmp::Ordering::Equal;

use scraper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: summarize the data
    for page_num in 1..3 {
        let url = format!("https://finance.yahoo.co.jp/stocks/ranking/up?page={}", page_num);
        let body = reqwest::blocking::get(url)?.text()?;
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("span._1QEp9BsV").unwrap();
        let elements = document.select(&selector);

        let mut rates = Vec::new();
        elements.for_each(|e| rates.push(e.text().next().unwrap().parse::<f32>().unwrap()));

        println!("{}", median(&mut rates));
    }
    Ok(())
}

fn median(numbers: &mut [f32]) -> f32 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    let mid = numbers.len() / 2;
    return numbers[mid];
}
