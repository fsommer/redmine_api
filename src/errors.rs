//! Generic error chain.
error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Reqwest(::reqwest::Error);
    }
}
