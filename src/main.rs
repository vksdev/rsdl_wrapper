const DEFAULT_YT_FORMAT: &str = "best";
const DEFAULT_VK_FORMAT: &str = "url720";

fn main() {
    // parse arguments
    let (link, args) = parse_arguments();
    println!("[rsdl] Given link: {}", link);

    // prepare base command
    let mut cmd = std::process::Command::new("cmd");
    cmd.arg("/C");
    let terminal_cmd;

    // get terminal commands in each case
    if link.contains("https://www.youtube.com") {
        terminal_cmd = get_youtube_cmd(link, args);
    } else if link.contains("https://www.vk.com/video") {
        terminal_cmd = get_vk_cmd(link, args);
    } else {
        terminal_cmd = get_aria2c_cmd(link);
    }

    // make full command and run it
    cmd.arg(&terminal_cmd);
    println!("[rsdl] Terminal Command: {}", &terminal_cmd);
    let command_failure_message = format!("!! Failed to run process: \n\t{} !!", terminal_cmd);
    let status = cmd.status().expect(&command_failure_message);
    println!("[rsdl] Process finished with: {}", status);
}

fn parse_arguments() -> (String, Option<(String, Option<String>)>) {
    let mut all_args: Vec<String> = std::env::args().collect();
    all_args.remove(0);

    if all_args.is_empty() {
        println!("!! Arguments not in right format !!");
        show_help_and_exit();
    }
    if all_args[0] == "-h" {
        show_help_and_exit();
    }
    let link = all_args.remove(0);
    if all_args.is_empty() {
        (link, None)
    } else {
        let flag = all_args.remove(0);
        match flag.as_str() {
            "-f" => {
                if all_args.is_empty() {
                    std::panic!("!! Give atlest one format !!")
                } else {
                    (link, Some(("-f".to_string(), Some(all_args.remove(0)))))
                }
            }
            "-F" => (link, Some(("-F".to_string(), None))),
            _ => return (link, None),
        }
    }
}

fn get_youtube_cmd(link: String, args: Option<(String, Option<String>)>) -> String {
    let cmd_first_part = "youtube-dl --external-downloader aria2c";
    match args {
        None => format!("{} {} -f {}", cmd_first_part, link, DEFAULT_YT_FORMAT),
        Some((s1, op)) => match op {
            None => format!("{} {} {}", cmd_first_part, link, s1),
            Some(s2) => format!("{} {} {} {}", cmd_first_part, link, s1, s2),
        },
    }
}

fn get_vk_cmd(link: String, args: Option<(String, Option<String>)>) -> String {
    let cmd_first_part = "youtube-dl --external-downloader aria2c";
    match args {
        None => format!("{} {} -f {}", cmd_first_part, link, DEFAULT_VK_FORMAT),
        Some((s1, op)) => match op {
            None => format!("{} {} {}", cmd_first_part, link, s1),
            Some(s2) => format!("{} {} {} {}", cmd_first_part, link, s1, s2),
        },
    }
}

fn get_aria2c_cmd(link: String) -> String {
    let cmd_first_part = "aria2c -c -j 1 -x 16 -s 16";
    format!("{} {}", cmd_first_part, link)
}

fn show_help_and_exit() {
    let str_one = r#"
rsdl 0.1.0
A wrapper around aria2c and youtube-dl.
Aria2 and youtube-dl must be already installed on system
and must be discoverable by cmd.

Usage: rsdl LINK [OPTIONAL_ARGS]

All Usage Examples:
	rsdl LINK -f DESIRED_FORMAT
		: to download videos in DESIRED_FORMAT using youtube-dl.
	rsdl LINK -F
		: to download videos info about available formats using youtube-dl.
	rsdl LINK
		: to download direct link content or torrents using aria2c but
		also to download videos from youtube and vk.
		Hard Coded or Defult formats are used to download videos using youtube-dl.
		Default formats are: youtube(best), vk(url720).
"#;
    println!("{}", str_one);
    std::process::exit(0);
}
