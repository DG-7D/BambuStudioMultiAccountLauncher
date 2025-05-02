use std::{error, io::Write};

const BAMBU_CONFIG_DIR: &str = "%USERPROFILE%\\AppData\\Roaming\\BambuStudio";
const BAMBU_CONFIG_FILE: &str = "BambuNetworkEngine.conf";
const BAMBU_EXE: &str = "bambu-studio.exe";
const SEPARATOR_CONF_PROFILE: &str = "_";

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    println!("Profile: {:?}", config.profile);
    println!("Ohters: {:?}", config.others);
    println!("Current: {:?}", get_current_profile()?);

    if config.profile.is_some() {
        kill_bambu();
        set_profile(config.profile.unwrap())?;
        println!("Current: {:?}", get_current_profile()?);
        return Ok(());
    }

    println!("{:?}", get_profile_list()?);
    return Ok(());
}

fn bambu_config_dir() -> std::path::PathBuf {
    return std::path::PathBuf::from(
        &BAMBU_CONFIG_DIR.replace("%USERPROFILE%", &std::env::var("USERPROFILE").unwrap()),
    );
}

fn get_profile_list() -> Result<Vec<String>, std::io::Error> {
    let mut profile_list = Vec::<String>::new();
    let config_dir = bambu_config_dir();
    for entry in std::fs::read_dir(config_dir)? {
        let entry = entry?;
        let file_name_osstr = entry.file_name();
        let file_name = file_name_osstr.to_str().unwrap();
        if entry.file_type()?.is_file() && file_name.starts_with(BAMBU_CONFIG_FILE) {
            if file_name[BAMBU_CONFIG_FILE.len()..].starts_with(SEPARATOR_CONF_PROFILE) {
                profile_list.push(
                    file_name[BAMBU_CONFIG_FILE.len() + SEPARATOR_CONF_PROFILE.len()..].to_string(),
                );
            }
        };
    }
    return Ok(profile_list);
}

fn get_current_profile() -> Result<String, std::io::Error> {
    let config_dir = bambu_config_dir();
    let config_file = config_dir.join(BAMBU_CONFIG_FILE);
    let current_config = match std::fs::read(&config_file) {
        Ok(config) => config,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!(
                "{} not found. Creating empty one.",
                &config_file.to_str().unwrap()
            );
            std::fs::File::create(&config_file)?;
            return Ok(String::from(""));
        }
        Err(error) => return Err(error),
    };
    for profile in get_profile_list()? {
        let file_name = format!("{}{}{}", BAMBU_CONFIG_FILE, SEPARATOR_CONF_PROFILE, profile);
        let checking_config = std::fs::read(config_dir.join(file_name))?;
        if checking_config == current_config {
            return Ok(profile);
        }
    }
    return Ok(String::from(""));
}

fn is_bambu_running() -> bool {
    let stdout = std::process::Command::new("tasklist")
        .args(["/fi", &format!("imagename eq {}", BAMBU_EXE)])
        .output()
        .unwrap()
        .stdout;
    let stdout = String::from_utf8(stdout).unwrap();
    return stdout.contains(BAMBU_EXE);
}

fn kill_bambu() {
    use getch_rs::{Getch, Key};
    const TIMEOUT: u32 = 5;

    if !is_bambu_running() {
        return;
    }
    std::process::Command::new("taskkill")
        .args(["/im", BAMBU_EXE])
        .spawn()
        .unwrap();
    for _ in 0..TIMEOUT {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if !is_bambu_running() {
            return;
        }
    }
    println!("{} was not closed withn {} seconds.", BAMBU_EXE, TIMEOUT);
    println!("");
    println!("Please choose one of the following options:");
    println!("m: Close mannually");
    println!("k: Kill forcefully");
    println!("q: Cancel and exit");

    loop {
        let key: Key;
        // Getchを破棄するようにしないとCtrl+Cとかができなくなる
        {
            let g = Getch::new();
            key = g.getch().unwrap();
        }
        match key {
            Key::Char('m') => {
                print!("Wait for {} mannually closed .", BAMBU_EXE);
                std::io::stdout().flush().unwrap();
                while is_bambu_running() {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    print!(" .");
                    std::io::stdout().flush().unwrap();
                }
                println!();
                return;
            }
            Key::Char('k') => {
                println!("Killing {} forcefully.", BAMBU_EXE);
                std::process::Command::new("taskkill")
                    .args(["/f", "/im", BAMBU_EXE])
                    .spawn()
                    .unwrap();
                while is_bambu_running() {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                return;
            }
            Key::Char('q') => {
                println!("Cancelled.");
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

fn set_profile(profile_name: String) -> Result<(), std::io::Error> {
    let config_dir = bambu_config_dir();
    std::fs::remove_file(config_dir.join(BAMBU_CONFIG_FILE))?;
    let file_name = format!(
        "{}{}{}",
        BAMBU_CONFIG_FILE, SEPARATOR_CONF_PROFILE, profile_name
    );
    std::fs::hard_link(
        config_dir.join(file_name),
        config_dir.join(BAMBU_CONFIG_FILE),
    )?;
    return Ok(());
}

pub struct Config {
    profile: Option<String>,
    others: Vec<String>,
}
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mut profile = None;
        let mut others = Vec::new();

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
                others.push(arg);
            }
        }

        return Ok(Config { profile, others });
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
        assert_eq!(config.others, Vec::<String>::new());
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
        assert_eq!(config.others, Vec::<String>::new());
    }
    #[test]
    fn args_only_others() {
        let args = vec![String::from("test.exe"), String::from("filename.3mf")].into_iter();
        let config = Config::new(args).unwrap();
        assert_eq!(config.profile, None);
        assert_eq!(config.others, vec![String::from("filename.3mf")]);
    }
    #[test]
    fn args_profile_others() {
        let args = vec![
            String::from("test.exe"),
            String::from("--profile"),
            String::from("profile"),
            String::from("filename.3mf"),
        ]
        .into_iter();
        let config = Config::new(args).unwrap();
        assert_eq!(config.profile, Some(String::from("profile")));
        assert_eq!(config.others, vec![String::from("filename.3mf")]);
    }
    #[test]
    fn args_othrers_profile() {
        let args = vec![
            String::from("test.exe"),
            String::from("filename.3mf"),
            String::from("--profile"),
            String::from("profile"),
        ]
        .into_iter();
        let config = Config::new(args).unwrap();
        assert_eq!(config.profile, Some(String::from("profile")));
        assert_eq!(config.others, vec![String::from("filename.3mf")]);
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
}
