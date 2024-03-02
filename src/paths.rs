use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Path {
    pub path: &'static str,
    pub text: &'static str,
    pub cta: &'static str,
    pub choices: Vec<&'static str>,
}

macro_rules! path {
    ($path: expr, $cta: expr, $text: expr, $choices: expr) => {
        (
            $path,
            Path {
                path: $path,
                cta: $cta,
                text: $text,
                choices: $choices,
            },
        )
    };
}

lazy_static! {
    pub static ref PATHS: HashMap<&'static str, Path> = [
        path! {
            "/",
            "Not important",
            "You are born as either a shelterd prince or princess.",
            vec!["/prince", "/princess"]
        },
        path! {
            "/prince",
            "Be a prince",
            "You are born to a King who needed you. You are doomed to three fates.",
            vec![]
        },
        path! {
            "/princess",
            "Be a princess",
            "You are born to a Syrian Prince who looks you up. ",
            vec![]
        },
    ]
    .into();
}
