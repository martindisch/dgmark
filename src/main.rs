fn main() {
    let input = r#"Hi there with [[productlist: 1|2|20]] and [[productlist:20]]. We have a quote [[quote:Some text is nice"The source"]] too."#;

    let (_, parsed) = dgmark::parse(input).unwrap();
    println!("{:?}", parsed);

    let formatted = parsed
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", formatted);
}
