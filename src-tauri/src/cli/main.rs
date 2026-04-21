use clap::Parser;

fn main() {
    let args = app_lib::cli::CliArgs::parse();
    let pretty = args.pretty;
    let result = app_lib::cli::run(args);

    println!("{}", result.render(pretty));
    std::process::exit(result.code());
}
