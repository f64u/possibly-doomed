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
            "You are to be born as either a shelterd prince or princess.",
            vec!["/prince", "/princess"]
        },
        path! {
            "/prince",
            "Be a prince",
            "You are born to an Egyptian King after he had trouble conceiving a son. However, your fate was determined at birth by the seven Hathors so that you die either by snake, corcodile, or even dog. Hearing this, the King locks you up in a stone house in the desert and forbids your leaving. The days pass by. You have never seen the outside.",
            vec!["/prince/stay", "/prince/leave"]
        },
        path! {
            "/prince/stay",
            "You stay locked up in the stone house.",
            "You stay locked up in the stone house. One day, you see from afar a man walking with a creature. You ask your servant what it is that the man is walking this, and he answers that it is a dog.",
            vec!["/prince/stay/ask-for-dog", "/prince/stay/no-dog"]
        },
        path! {
            "/prince/leave",
            "You ask your father to leave.",
            "You ask your father to leave, but he refuses since you're too young.",
            vec!["/prince/stay"]
        }, // End of path
        path! {
            "/prince/stay/ask-for-dog",
            "You ask the servant to bring you a dog.",
            "You ask the servant to bring you a dog. Even though your fate mentioned a dog ending your life, you have been locked up in the stone house for too long and you feel lonely.",
            vec!["/prince/stay/ask-for-dog/pet-dog"]
        },
        path! {
            "/prince/stay/no-dog",
            "You nod in fear.",
            "Even though you have been locked up in the stone house for too long and have been feeling lonely, you're aware that a dog could end your life according to your fate. You stay away from dogs.\nHello",
            vec![]
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog",
            "You pet the dog.",
            "You pet the dog and raise it. He seems to like you.\nThe days pass by and you grow more wary of your way of life.",
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
