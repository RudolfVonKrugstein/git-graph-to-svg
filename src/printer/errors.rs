use std::io::BufWriter;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        BufferError(::std::io::IntoInnerError<BufWriter<Vec<u8>>>);
        Utf8Error(::std::string::FromUtf8Error);
    }
}
