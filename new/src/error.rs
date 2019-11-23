use reqwest;

error_chain!{
    foreign_links {
        Io(::std::io::Error);
        ReqwestUrl(reqwest::UrlError);
        Reqwest(reqwest::Error);
    }
}
