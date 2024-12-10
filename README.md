
# Parfumo Top Perfumes Scraper

This Rust project scrapes the top perfumes listed on [Parfumo](https://www.parfumo.com) for different categories (Women, Men, Unisex) and saves the results into CSV files. The output includes the perfume's name, brand, URL, and image source.

---

## How It Works

1. **HTTP Request**: Sends a GET request to the Parfumo website for each category.
2. **HTML Parsing**: Parses the returned HTML using the `scraper` crate.
3. **Data Extraction**:
   - Extracts perfume details using CSS selectors.
   - Handles missing or incomplete data gracefully.
4. **CSV Export**: Writes the extracted data into category-specific CSV files.


---

## Features

- **Scrape Perfume Data**: Extracts name, brand, URL, and image source for each perfume.
- **Handles Multiple Categories**: Scrapes data for Women, Men, and Unisex categories.
- **CSV Export**: Saves the scraped data in neatly formatted CSV files.

---

## Prerequisites

### Install Rust
Ensure that you have Rust installed on your system. If not, you can install it using [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Add Required Dependencies
This project uses the following Rust crates:
- [`reqwest`](https://crates.io/crates/reqwest): For making HTTP requests.
- [`scraper`](https://crates.io/crates/scraper): For parsing and scraping HTML content.
- [`csv`](https://crates.io/crates/csv): For writing CSV files.

---

## Usage

### Clone the Repository
Clone this repository to your local machine:
```bash
git clone https://github.com/yourusername/parfumo-scraper.git
cd parfumo-scraper
```

### Run the Scraper
Run the scraper using `cargo`:
```bash
cargo run
```

The scraper will fetch perfume data from the following pages:
- `/Tops/Women`
- `/Tops/Men`
- `/Tops/Unisex`

Each category's data is saved to a separate CSV file:
- `perfumes_Women.csv`
- `perfumes_Men.csv`
- `perfumes_Unisex.csv`

---

## Example Output

### CSV Content
Each CSV file will look like this:

| Name              | Brand          | URL                                  | Image                          |
|--------------------|----------------|--------------------------------------|--------------------------------|
| Chanel No. 5       | Chanel         | https://www.parfumo.com/Perfumes/... | https://imageurl.com/...       |
| Sauvage            | Dior           | https://www.parfumo.com/Perfumes/... | https://imageurl.com/...       |

## Author

Created by [Antonio Djigo](https://x.com/brownio_).
