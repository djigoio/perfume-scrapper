use csv::WriterBuilder;
use reqwest;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
struct Perfume {
    name: String,
    brand: String,
    url: String,
    image: String,
    score: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let types = ["/Tops/Women", "/Tops/Men", "/Tops/Unisex"];

    for page in &types {
        let url = format!("https://www.parfumo.com/Perfumes{}", page);
        println!("{url}");
        let response = client
        .get(&url)
        .send()?;

        let html_content = response.text()?;

        // Print the entire HTML content for debugging
        //println!("HTML Content: {}", &html_content);

        let document = Html::parse_document(&html_content);

        let card_selector = Selector::parse("div.col-normal").unwrap();

        let image_selector = Selector::parse("picture img").unwrap();
        let name_url_selector = Selector::parse("div.name a").unwrap();
        let brand_selector = Selector::parse("span.brand a").unwrap();
        let score_selector = Selector::parse("div.av_scent").unwrap();

        let mut perfumes: Vec<Perfume> = Vec::new();

        let matching_cards: Vec<_> = document.select(&card_selector).collect();
        // Debug: print number of matching cards
        //println!("Number of matching cards: {}", matching_cards.len());

        for card in matching_cards {
            // Debug: print the entire card HTML
            //println!("Card HTML: {}", card.html());

            // Extract image
            let image = match card.select(&image_selector).next() {
                Some(img) => {
                    let src = img.value().attr("src").unwrap_or("No image source");
                    println!("Image source: {}", src);
                    src.to_string()
                }
                None => {
                    println!("No image found in card");
                    continue;
                }
            };

            // Extract name and URL
            let name_element = match card.select(&name_url_selector).next() {
                Some(element) => {
                    let name = element.text().collect::<String>();
                    let url = element.value().attr("href").unwrap_or("No URL");
                    println!("Name: {}, URL: {}", name, url);
                    (name, url)
                }
                None => {
                    println!("No name/URL found in card");
                    continue;
                }
            };

            // Extract brand
            let brand = match card.select(&brand_selector).next() {
                Some(brand_el) => {
                    let brand_text = brand_el.text().collect::<String>();
                    println!("Brand: {}", brand_text);
                    brand_text
                }
                None => {
                    println!("No brand found in card");
                    continue;
                }
            };

            let score = match card.select(&score_selector).next() {
                Some(score_el) => {
                    let score_text = score_el.text().collect::<String>().trim().to_string();
                    println!("score: {}", score_text); 
                    score_text
                }
                None => {
                    println!("No score found in card");
                    continue;
                }
            };
            
            // Create perfume entry
            let perfume = Perfume {
                name: name_element.0.trim().to_owned(),
                brand: brand.trim().to_owned(),
                url: name_element.1.to_string(),
                image,
                score,
            };

            perfumes.push(perfume);
        }
        println!("{page}");

        let sanitized_filename = page.strip_prefix("/Tops/").unwrap_or(page);
        let file_name: String = format!("perfumes_{}.csv", sanitized_filename);

        println!("{file_name}");

        // Create CSV writer
        let mut writer = WriterBuilder::new()
            .has_headers(true)
            .from_path(&file_name)?;

        // Write headers
        writer.write_record(&["Name", "Brand", "URL", "Image", "Score/Users"])?;
        // Write perfume data
        for perfume in &perfumes {
            writer.write_record(&[
                perfume.name.clone(),
                perfume.brand.clone(),
                perfume.url.clone(),
                perfume.image.clone(),
                perfume.score.clone(),
            ])?;
        }

        writer.flush()?;

        println!("Scraped {} perfumes", perfumes.len());
    }
    Ok(())
}
