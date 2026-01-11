use leptos::prelude::RwSignal;

#[derive(Clone, Debug)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub date: String,
    pub series: Option<String>,
    pub publisher: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BooksContext {
    pub library: RwSignal<Vec<Book>>,
}

impl Default for BooksContext {
    fn default() -> Self {
        Self {
            library: RwSignal::new(vec![
                Book {
                    id: 82,
                    title: "1174 Raising Arianna ebook".to_string(),
                    author: "Unknown".to_string(),
                    date: "20 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 83,
                    title: "Training Spies: CIA & MI6 with Bonus Material".to_string(),
                    author: "Brian Nash-Fritz".to_string(),
                    date: "28 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 84,
                    title: "Text-Taking Strategies".to_string(),
                    author: "Judi Kesselman-".to_string(),
                    date: "20 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 85,
                    title: "Unexpected Journey".to_string(),
                    author: "David Crane".to_string(),
                    date: "20 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 86,
                    title: "The Honor of the Queen".to_string(),
                    author: "David Weber".to_string(),
                    date: "20 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 87,
                    title: "Star Wars: Rise of the Empire -- Religion".to_string(),
                    author: "John Jackson ...".to_string(),
                    date: "17 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 88,
                    title: "James Cameron AVATAR-far".to_string(),
                    author: "Jon Landau".to_string(),
                    date: "19 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 89,
                    title: "Night Shift".to_string(),
                    author: "Elizabeth Moon".to_string(),
                    date: "17 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 90,
                    title: "On Basilisk Station".to_string(),
                    author: "David Weber".to_string(),
                    date: "20 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 91,
                    title: "Warbreaker".to_string(),
                    author: "Brandon Sand...".to_string(),
                    date: "17 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 92,
                    title: "Getting-Start Guide".to_string(),
                    author: "John Schember".to_string(),
                    date: "19 Jun".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 93,
                    title: "The Briar King".to_string(),
                    author: "Greg Keyes et ...".to_string(),
                    date: "25 Feb".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 94,
                    title: "To Sail Beyond the Wind".to_string(),
                    author: "Percile Rouleau".to_string(),
                    date: "05 Jan".to_string(),
                    series: None,
                    publisher: None,
                },
                Book {
                    id: 95,
                    title: "_MaxLinuxDevXvl03".to_string(),
                    author: "EDO SEGAL".to_string(),
                    date: "28 Dec".to_string(),
                    series: None,
                    publisher: None,
                },
            ]),
        }
    }
}

impl BooksContext {}
