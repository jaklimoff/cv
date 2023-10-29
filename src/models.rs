use serde::{Deserialize, Serialize};
use serde::de::Expected;

// Models for the CV based on community driven
// open source initiative to create JSON-based standard for resumes.
// https://jsonresume.org/schema/

/// keyword should be included in the macro
/// [ref](https://rust-lang.github.io/api-guidelines/macros.html#input-syntax-is-evocative-of-the-output-c-evocative)
macro_rules! pub_struct {
    ($ke:ident $name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)] // ewww
        #[serde(rename_all = "camelCase")]
        pub $ke $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(struct CV {
    basics: Basics,
    expertise: Vec<Expertise>,
    work: Vec<Work>,
    volunteer: Vec<Work>,
    education: Vec<Education>,
    awards: Vec<Award>,
    certificates: Vec<Certificate>,
    publications: Vec<Publication>,
    skills: Vec<Skill>,
    languages: Vec<Language>,
    interests: Vec<Interest>,
    references: Vec<Reference>,
    projects: Vec<Project>,
});

pub_struct!(struct Basics {
    first_name: Option<String>,
    last_name: Option<String>,
    label: Option<String>,
    image: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    url: Option<String>,
    summary: Option<String>,
    location: Location,
    profiles: Vec<Profile>,
});

pub_struct!(struct Location {
    address: Option<String>,
    postal_code: Option<String>,
    city: Option<String>,
    country: Option<String>,
    country_code: Option<String>,
    region: Option<String>,
});

pub_struct!(struct Expertise {
    title: Option<String>,
    summary: Option<String>,
});

pub_struct!(struct Profile {
    network: Option<String>,
    username: Option<String>,
    url: Option<String>,
});

pub_struct!(struct Work {
    name: Option<String>,
    location: Option<String>,
    position: Option<String>,
    url: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    summary: Option<String>,
    highlights: Vec<String>,
});

pub_struct!(struct Education {
    institution: Option<String>,
    url: Option<String>,
    area: Option<String>,
    study_type: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    score: Option<String>,
    courses: Vec<String>,
});

pub_struct!(struct Award {
    title: Option<String>,
    date: Option<String>,
    awarder: Option<String>,
    summary: Option<String>,
});

pub_struct!(struct Certificate {
    name: Option<String>,
    date: Option<String>,
    issuer: Option<String>,
    url: Option<String>,
});

pub_struct!(struct Publication {
    name: Option<String>,
    publisher: Option<String>,
    release_date: Option<String>,
    url: Option<String>,
    summary: Option<String>,
});

pub_struct!(struct Skill {
    name: Option<String>,
    level: Option<String>,
    keywords: Vec<String>,
});

pub_struct!(struct Language {
    language: Option<String>,
    fluency: Option<String>,
});

pub_struct!(struct Interest {
    name: Option<String>,
    keywords: Vec<String>,
});

pub_struct!(struct Reference {
    name: Option<String>,
    reference: String,
});

pub_struct!(struct Project {
    name: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    description: Option<String>,
    highlights: Vec<String>,
    url: Option<String>,
});
