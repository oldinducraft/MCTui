#[derive(Default, PartialEq, Clone)]
pub enum Field {
    #[default]
    Username,
    Password,
}

pub type ErrorMessage = Option<String>;
