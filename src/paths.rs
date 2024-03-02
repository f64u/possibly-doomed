use std::collections::HashMap;

use crate::path::Path;

use lazy_static::lazy_static;

macro_rules! path {
    ($path: expr, $text: expr, $choices: expr) => {
        (
            $path,
            Path {
                path: $path,
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
            "You are born as either a shelterd prince or princess.",
            vec!["/prince", "/princess"]
        },
        path! {
            "/prince",
            "You are born to a King who needed you. You are doomed to three fates.",
            vec![]
        }
    ]
    .into();
}
