struct Symbol;

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64)
    } else if args.len() == 2 {
        run_file(args[1].to_string())?;
    } else {
        run_prompt()?;
    }
    Ok(())
}
