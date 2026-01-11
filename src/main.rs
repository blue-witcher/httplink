use std::env;

fn create_http_link(link: &str) {
    println!("<html>");
    println!("<head>");
    println!("<meta http-equiv=\"refresh\" content=\"0; url={}\" />", link);
    println!("</head>");
    println!("</html>");
}

fn main() {
    // dump cli arguments to args variable
    let args: Vec<String> = env::args().skip(1).collect();
    // check if only one argument was given
    if args.iter().count() == 1 {
        let link = &args[0];
        create_http_link(link);
    } else {
        eprintln!("You need to provide at least one URL as a command-line argument.")
    }
}
