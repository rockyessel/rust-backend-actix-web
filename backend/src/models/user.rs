use bson::{from_document, to_document, Document};
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub name: String,
    pub username: String,
    pub email: String,
    pub bio: String,
    pub password: String,
}

impl From<User> for Document {
    fn from(user: User) -> Self {
        to_document(&user).expect("Failed to convert user to BSON document")
    }
}

impl TryFrom<Document> for User {
    type Error = bson::de::Error;

    fn try_from(doc: Document) -> Result<Self, Self::Error> {
        from_document(doc)
    }
}

// impl User {
//     pub fn new(
//         name: String,
//         username: String,
//         social_links: SocialLinks,
//         email: String,
//         bio: String, comments: Vec<String> ) -> Self {
//         User {
//             name,
//             username,
//             social_links,
//             email,
//             bio,
//             comments,
//         }
//     }

//     pub fn display(&self) {
//         println!("Name: {}", self.name);
//         println!("Username: {}", self.username);
//         println!("Email: {}", self.email);
//         println!("Bio: {}", self.bio);
//         println!("Social Links:");
//         if let Some(github) = &self.social_links.github {
//             println!(" - Github: {}", github);
//         }
//         if let Some(twitter) = &self.social_links.twitter {
//             println!(" - Twitter: {}", twitter);
//         }
//         if let Some(linkedin) = &self.social_links.linkedin {
//             println!(" - LinkedIn: {}", linkedin);
//         }
//         if let Some(youtube) = &self.social_links.youtube {
//             println!(" - YouTube: {}", youtube);
//         }
//         println!("Comments: {:?}", self.comments);
//     }
// }
