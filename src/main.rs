use dgmark::Element;

fn main() {
    let input = "Hi there with [[productlist: 1|2|20]] and [[productlist:20]]";
    let (_, parsed) = dgmark::parse(input).unwrap();
    println!("{:?}", parsed);
}
