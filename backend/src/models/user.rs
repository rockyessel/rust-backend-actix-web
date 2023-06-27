use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub email: String,
    pub bio: String,
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
