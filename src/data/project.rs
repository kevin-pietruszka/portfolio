#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub image_path: String,
    pub is_public: bool,
    pub github_link: String,
}


impl Project {
    pub fn data() -> Vec<Project> {
        vec![
            Project {
                title: String::from("Pacman AI"),
                description: String::from("AI to play Pacman. It is pretty cool but this is just a test to see what happens with longer text"),
                image_path: String::from("img/pacman.png"),
                is_public: false,
                github_link: String::new(),
            },
            Project {
                title: String::from("Pacman AI"),
                description: String::from("AI to play Pacman"),
                image_path: String::from("img/pacman.png"),
                is_public: false,
                github_link: String::new(),
            },
            Project {
                title: String::from("Pacman AI"),
                description: String::from("AI to play Pacman"),
                image_path: String::from("img/pacman.png"),
                is_public: false,
                github_link: String::new(),
            },
            Project {
                title: String::from("Pacman AI"),
                description: String::from("AI to play Pacman"),
                image_path: String::from("img/pacman.png"),
                is_public: false,
                github_link: String::new(),
            },
            Project {
                title: String::from("Pacman AI"),
                description: String::from("AI to play Pacman"),
                image_path: String::from("img/pacman.png"),
                is_public: false,
                github_link: String::new(),
            },
            Project {
                title: String::from("Pacman AI"),
                description: String::from("AI to play Pacman"),
                image_path: String::from("img/pacman.png"),
                is_public: false,
                github_link: String::new(),
            },
            Project {
                title: String::from("Pacman AI"),
                description: String::from("AI to play Pacman"),
                image_path: String::from("img/pacman.png"),
                is_public: false,
                github_link: String::new(),
            },
        ]
    }
}