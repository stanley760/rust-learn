#[test]
fn convert() {
    use std::fs;
    let url = "http://www.rust-lang.org/";
    let output = "rust.md";
    println!("fetching url:{}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("converting to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();

    println!(
        "Converted done,And the markdown has been saved in {}.",
        output
    );
}
