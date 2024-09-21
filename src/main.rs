mod connections;
mod session;
mod log;
mod args;
mod executor;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    const CONNECTION_NAME: &str = "MEO-WiFi";
    let (args, info) = args::new()?.unwrap();

    if let Some(info) = info {
        println!("{}", info);
        return Ok(());
    }

    let args = args.unwrap();

    let network_manager = connections::Nmcli::new();
    let session = session::Legacy::new(args.user(), args.pass())?;

    let executor: Box<dyn executor::Trait> = if args.is_one_shot() {
        Box::new(executor::Oneshot::new(&network_manager, &session, CONNECTION_NAME))
    } else {
        Box::new(executor::Continuous::new(&network_manager, &session, CONNECTION_NAME))
    };

    executor.execute()?;
    Ok(())
}

fn main() -> () {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
