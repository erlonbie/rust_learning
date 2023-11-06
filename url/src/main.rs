use url::Url;

fn parse_url(url_string: &str) -> Result<(), url::ParseError> {
    let url = Url::parse(url_string)?;

    println!("Scheme: {}", url.scheme());
    println!("default port : {:?}", url.port_or_known_default());
    println!("Host: {}", url.host_str().unwrap_or(""));
    println!("Path: {}", url.path());

    Ok(())
}

fn main() {
    let url_string = "https://example.com/path/to/resource";
    parse_url(url_string).unwrap();
    println!("{}", url_string);
}
