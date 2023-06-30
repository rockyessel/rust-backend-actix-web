
pub mod models {
    pub mod user;
    mod package;
}


pub mod controllers {
    pub mod user;
    mod package;
}


pub mod routes {
    pub mod user;
    mod package;
}

pub mod services {
    pub mod db;
    
}

pub mod utils {
    pub mod helpers;
}

pub mod middleware {
    pub mod authentication;
}