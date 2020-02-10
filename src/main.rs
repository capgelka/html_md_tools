use html_md_tools::print_html;



fn get_request(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::blocking::get(url)?.text()?;
    Ok(res)
  
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    let html = get_request(url).unwrap();
    print_html(&html).unwrap()
}
