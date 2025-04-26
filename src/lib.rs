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
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mut profile = None;
        let mut filename = None;

        while let Some(arg) = args.next() {
            if arg == "--profile" {
                if profile.is_some() {
                    return Err("'--profile' option is not allowed more than '1' time(s).");
                }
                profile = match args.next() {
                    Some(val) => Some(val),
                    None => return Err("Value expected for '--profile'."),
                };
                continue;
            } else {
                if filename.is_some() {
                    return Err("Default option is not allowed more than '1' time(s).");
                }
                filename = Some(arg);
            }
        }

        return Ok(Config { profile, filename });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn args_empty() {
        let args = vec![String::from("test.exe")].into_iter();
        let config = Config::new(args).unwrap();
        assert_eq!(config.profile, None);
        assert_eq!(config.filename, None);
    }
    #[test]
    fn args_only_profile() {
        let args = vec![
            String::from("test.exe"),
            String::from("--profile"),
            String::from("profile"),
        ]
        .into_iter();
        let config = Config::new(args).unwrap();
        assert_eq!(config.profile, Some(String::from("profile")));
        assert_eq!(config.filename, None);
    }
    #[test]
    fn args_only_filename() {
        let args = vec![String::from("test.exe"), String::from("filename.3mf")].into_iter();
        let config = Config::new(args).unwrap();
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
        ]
        .into_iter();
        let config = Config::new(args).unwrap();
        assert_eq!(config.profile, Some(String::from("profile")));
        assert_eq!(config.filename, Some(String::from("filename.3mf")));
    }
    #[test]
    #[should_panic(expected = "Value expected for '--profile'.")]
    fn args_profile_without_value() {
        let args = vec![String::from("test.exe"), String::from("--profile")].into_iter();
        Config::new(args).unwrap();
    }
    #[test]
    #[should_panic(expected = "'--profile' option is not allowed more than '1' time(s).")]
    fn args_multiple_profile() {
        let args = vec![
            String::from("test.exe"),
            String::from("--profile"),
            String::from("profile1"),
            String::from("--profile"),
            String::from("profile2"),
        ]
        .into_iter();
        Config::new(args).unwrap();
    }
    #[test]
    #[should_panic(expected = "Default option is not allowed more than '1' time(s).")]
    fn args_multiple_filename() {
        let args = vec![
            String::from("test.exe"),
            String::from("filename1.3mf"),
            String::from("filename2.3mf"),
        ]
        .into_iter();
        Config::new(args).unwrap();
    }
}
