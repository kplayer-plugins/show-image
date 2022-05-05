extern crate kplayer_rust_wrap;

use kplayer_rust_wrap::kplayer;

struct ShowOverlay {}
impl ShowOverlay {
    fn new() -> Self {
        ShowOverlay {}
    }
}

impl kplayer::plugin::BasePlugin for ShowOverlay {
    fn get_name(&self) -> String {
        String::from("show-image")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("x=0"));
        args.push(String::from("y=0"));
        args
    }
    fn get_allow_custom_args(&self) -> Vec<&'static str> {
        vec!["x", "y"]
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("overlay")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        Ok(true)
    }
}

// movie
struct ShowMovie {}
impl ShowMovie {
    fn new() -> Self {
        ShowMovie {}
    }
}

impl kplayer::plugin::BasePlugin for ShowMovie {
    fn get_name(&self) -> String {
        String::from("show-image")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("filename=exmaple.png"));
        args
    }
    fn get_allow_custom_args(&self) -> Vec<&'static str> {
        vec!["filename"]
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("movie")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        for (key, value) in _args {
            // validate image file exist
            if key == String::from("filename") {
                if !kplayer::util::os::file_exist(&value) {
                    self.print_log(
                        kplayer::util::os::PrintLogLevel::ERROR,
                        format!("image file not eixst: {}", &value).as_str(),
                    );
                    return Err("image file not exist");
                }
            }
        }
        Ok(true)
    }
}

kplayer_rust_wrap::export!(ShowOverlay, ShowMovie);
