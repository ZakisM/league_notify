pub trait StringExt {
    fn title_case(self) -> String;
}

impl StringExt for String {
    fn title_case(self) -> String {
        self[..1].to_uppercase() + &self[1..].to_lowercase()
    }
}
