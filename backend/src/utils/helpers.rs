fn get_data_from_html_link() {

    let html_response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    let parsed_document = scraper::Html::parse_document(&html_response);

    let get_titles_response = scraper::Selector::parse("h3.lister-item-header>a")
        .unwrap();

    let titles = parsed_document.select(&get_titles_response)
    .map(|x| x.inner_html());
    
    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}