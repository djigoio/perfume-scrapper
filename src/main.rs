
use reqwest;
use scraper::{Html, Selector};
use csv::WriterBuilder;

#[derive(Debug, Clone)]
struct Perfume {
    name: String,
    brand: String,
    url: String,
    image: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with custom headers to mimic a browser
    let client = reqwest::blocking::Client::new();
    let types = ["/Tops/Women", "/Tops/Men", "/Tops/Unisex"];

    for page in &types {

    let url = format!("https://www.parfumo.com/Perfumes{}", page);
    println!("{url}");
    // Download the target HTML document with headers
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .send()?;
    
    // Get the HTML content from the request response
    let html_content = response.text()?;
    
    // Print the entire HTML content for debugging
    //println!("HTML Content: {}", &html_content);

    // Parse the HTML document
    let document = Html::parse_document(&html_content);

    // Selector for the main card
    let card_selector = Selector::parse("div.col-normal").unwrap();
    
    // Selectors for internal elements
    let image_selector = Selector::parse("picture img").unwrap();
    let name_url_selector = Selector::parse("div.name a").unwrap();
    let brand_selector = Selector::parse("span.brand a").unwrap();

    // Initialize vector to store perfumes
    let mut perfumes: Vec<Perfume> = Vec::new();

    // Debug: print number of matching cards
    let matching_cards: Vec<_> = document.select(&card_selector).collect();
    println!("Number of matching cards: {}", matching_cards.len());

    // Iterate through cards
    for card in matching_cards {
        // Debug: print the entire card HTML
        //println!("Card HTML: {}", card.html());

        // Extract image
        let image = match card.select(&image_selector).next() {
            Some(img) => {
                let src = img.value().attr("src").unwrap_or("No image source");
                println!("Image source: {}", src);
                src.to_string()
            },
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
            },
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
            },
            None => {
                println!("No brand found in card");
                continue;
            }
        };

        // Create perfume entry
        let perfume = Perfume {
            name: name_element.0.trim().to_owned(),
            brand: brand.trim().to_owned(),
            url: name_element.1.to_string(),
            image,
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
    writer.write_record(&["Name", "Brand", "URL", "Image"])?;

    // Write perfume data
    for perfume in &perfumes {
        writer.write_record(&[
            perfume.name.clone(), 
            perfume.brand.clone(), 
            perfume.url.clone(), 
            perfume.image.clone()
        ])?;
    }

    // Flush and close
    writer.flush()?;

    println!("Scraped {} perfumes", perfumes.len());

}
Ok(())

}