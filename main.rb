require 'net/http'
require 'nokogiri'
require 'csv'
require 'uri'

class Perfume
  attr_accessor :name, :brand, :url, :image, :score

  def initialize(name, brand, url, image, score)
    @name = name
    @brand = brand
    @url = url
    @image = image
    @score = score
  end
end

def scrape_perfumes
  # Define page types
  types = ["/Tops/Women", "/Tops/Men", "/Tops/Unisex"]

  # Loop through each page type
  types.each do |page|
    url = "https://www.parfumo.com/Perfumes#{page}"
    puts url

    uri = URI.parse(url)
    response = Net::HTTP.get_response(uri)

    # Parse HTML content
    doc = Nokogiri::HTML(response.body)

    # Initialize an array to hold perfumes
    perfumes = []

    # Find each perfume card
    doc.css('div.col-normal').each do |card|
      # Extract image
      image = card.at_css('picture img')&.[]('src')
      if image.nil?
        puts "No image found in card"
        next
      end
      puts "Image source: #{image}"

      # Extract name and URL
      name_url = card.at_css('div.name a')
      name = name_url&.text&.strip
      url = name_url&.[]('href')
      if name.nil? || url.nil?
        puts "No name/URL found in card"
        next
      end
      puts "Name: #{name}, URL: #{url}"

      # Extract brand
      brand = card.at_css('span.brand a')&.text&.strip
      if brand.nil?
        puts "No brand found in card"
        next
      end
      puts "Brand: #{brand}"

      # Extract score
      score = card.at_css('div.av_scent')&.text&.strip
      if score.nil? || score.empty?
        puts "No score found in card"
        next
      end
      puts "Score: #{score}"

      # Create a new perfume and add it to the list
      perfumes << Perfume.new(name, brand, url, image, score)
    end

    # Sanitize the filename
    sanitized_filename = page.sub('/Tops/', '')
    file_name = "perfumes_#{sanitized_filename}.csv"
    puts "File name: #{file_name}"

    # Create and write to CSV
    CSV.open(file_name, 'wb') do |csv|
      # Write headers
      csv << ['Name', 'Brand', 'URL', 'Image', 'Score/Users']

      # Write perfume data
      perfumes.each do |perfume|
        csv << [perfume.name, perfume.brand, perfume.url, perfume.image, perfume.score]
      end
    end

    puts "Scraped #{perfumes.size} perfumes"
  end
end

# Run the scraping function
scrape_perfumes
