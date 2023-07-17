use metadata::MetaData;
use metadata_derive::MetaData;

#[derive(MetaData)]
#[metadata(author = "sharkbound", /*version = "4.2"*/ /* version is optional and defaults to "0" */)]
struct Data {
    #[metadata(author = "lenny")]
    name: String,
    #[metadata(author = "james")]
    age: u32,
}


fn main() {
    let data = Data {
        name: "sharkbound".to_string(),
        age: 18,
    };
    println!("Data struct author: {:?}\nVersion: {:?}\nField authors: {:?}", data.author(), data.version(), data.field_authors());
    /*
        Data struct author: "sharkbound"
        Version: "0"
        Field authors: {"name": "lenny", "age": "james"}
    */
}
