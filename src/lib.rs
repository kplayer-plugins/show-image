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
    fn get_args(&self) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("x=0"));
        args.push(String::from("y=0"));
        args
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
    fn validate_user_args(&self, _args: &Vec<String>) -> std::result::Result<bool, &'static str> {
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
    fn get_args(&self) -> std::vec::Vec<std::string::String> {
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from("filename=exmaple.png"));
        args
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
    fn validate_user_args(&self, _args: &Vec<String>) -> std::result::Result<bool, &'static str> {
        for str in _args {
            let sp: Vec<&str> = str.split('=').collect();
            if sp.len() < 2 {
                self.print_log(
                    kplayer::util::os::PrintLogLevel::ERROR,
                    format!("validate args failed arg string: {}", str).as_str(),
                );
                return Err("args format error");
            }

            // validate image file exist
            if sp[0] == "filename" {
                if !kplayer::util::os::file_exist(sp[1].to_string()) {
                    self.print_log(
                        kplayer::util::os::PrintLogLevel::ERROR,
                        format!("image file not eixst: {}", sp[1]).as_str(),
                    );
                    return Err("image file not exist");
                }
            }
        }
        Ok(true)
    }
}

kplayer_rust_wrap::export!(ShowOverlay, ShowMovie);
