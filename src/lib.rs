use std::error;

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    println!("Profile: {:?}", config.profile);
    println!("Filename: {:?}", config.filename);
    return Ok(());
}

pub struct Config {
    profile: Option<String>,
    filename: Option<String>,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let mut profile = None;
        let mut filename = None;

        let mut iter = args.iter().skip(1);
        while let Some(arg) = iter.next() {
            if arg == "--profile" {
                println!("{:?}", profile);
                if profile.is_some() {
                    return Err("'--profile' option is not allowed multiple times");
                }
                profile = iter.next().cloned();
                if profile.is_none() {
                    return Err("Value expected for '--profile'");
                }
                continue;
            }
            if filename.is_none() {
                filename = Some(arg.clone());
                continue;
            } else {
                return Err("");
            }
        }

        return Ok(Config { profile, filename });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn args_only_profile() {
        let args = vec![
            String::from("test.exe"),
            String::from("--profile"),
            String::from("profile"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.profile, Some(String::from("profile")));
        assert_eq!(config.filename, None);
    }
    #[test]
    fn args_only_filename() {
        let args = vec![String::from("test.exe"), String::from("filename.3mf")];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.profile, None);
        assert_eq!(config.filename, Some(String::from("filename.3mf")));
    }
    #[test]
    fn args_profile_filename() {
        let args = vec![
            String::from("test.exe"),
            String::from("--profile"),
            String::from("profile"),
            String::from("filename.3mf"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.profile, Some(String::from("profile")));
        assert_eq!(config.filename, Some(String::from("filename.3mf")));
    }
    #[test]
    #[should_panic(expected = "Value expected for '--profile'")]
    fn args_profile_without_value() {
        let args = vec![String::from("test.exe"), String::from("--profile")];
        Config::new(&args).unwrap();
    }
    #[test]
    #[should_panic(expected = "'--profile' option is not allowed multiple times")]
    fn args_multiple_profile() {
        let args = vec![
            String::from("test.exe"),
            String::from("--profile"),
            String::from("profile1"),
            String::from("--profile"),
            String::from("profile2"),
        ];
        Config::new(&args).unwrap();
    }
    #[test]
    #[should_panic(expected = "")]
    fn args_multiple_filename() {
        let args = vec![
            String::from("test.exe"),
            String::from("filename1.3mf"),
            String::from("filename2.3mf"),
        ];
        Config::new(&args).unwrap();
    }
}
