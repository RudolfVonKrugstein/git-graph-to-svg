error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    foreign_links {
        Parse(::serde_yaml::Error);
    }
}
