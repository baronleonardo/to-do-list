pub mod cmd {
    use std::env;

    pub struct Args {
        pub file_path: String,
        pub x_pos: i32,
        pub y_pos: i32,
        pub width: i32,
        pub height: i32,
        cmd_name: String,
    }

    impl Args {
        pub fn parse() -> Args {
            let mut parsed_args = Args {
                file_path: String::new(),
                x_pos: 1100,
                y_pos: 50,
                width: 250,
                height: 300,
                cmd_name: String::new(),
            };

            let args: Vec<String> = env::args().collect();
            parsed_args.cmd_name = args[0].clone();

            match args.len() {
                3 if args[1] == "-f" => parsed_args.file_path = args[2].clone(),
                7 if args[1] == "-f" && args[3] == "-x" && args[5] == "-y" => {
                    parsed_args.file_path = args[2].clone();
                    parsed_args.x_pos = args[4].parse::<i32>().expect("Position must be int");
                    parsed_args.y_pos = args[6].parse::<i32>().expect("Position must be int");
                }
                11 if args[1] == "-f"
                    && args[3] == "-x"
                    && args[5] == "-y"
                    && args[7] == "-w"
                    && args[9] == "-h" =>
                {
                    parsed_args.file_path = args[2].clone();
                    parsed_args.x_pos = args[4].parse::<i32>().expect("Position must be int");
                    parsed_args.y_pos = args[6].parse::<i32>().expect("Position must be int");
                    parsed_args.width = args[8].parse::<i32>().expect("Position must be int");
                    parsed_args.height = args[10].parse::<i32>().expect("Position must be int");
                }
                _ => {
                    parsed_args.file_path = "".to_owned();
                    Args::help(&parsed_args.cmd_name);
                }
            }

            parsed_args
        }

        fn help(cmd_name: &String) {
            println!(
                "Help:\n{} -f <file path> [[-x <x pos>] [-y <y pos>]] [[-w <width>] [-h <height>]]",
                cmd_name
            );
            std::process::exit(0);
        }
    }
}