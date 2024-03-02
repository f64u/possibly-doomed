#[derive(Clone, Debug)]
pub struct Path {
    pub path: &'static str,
    pub text: &'static str,
    pub choices: Vec<&'static str>,
}
