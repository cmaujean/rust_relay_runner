use rppal::gpio::Gpio;
use std::error::Error;
use std::{env, process, thread};
use std::time::Duration;

fn print_help(msg: &str) {
  println!("Error: {}", msg);
  println!("Usage: relay_runner <relay number> <duration in seconds>");
} 

fn pins(relay_num: u8) -> Result<u8, u8> {
  match relay_num {
    1 => Ok(26),
    2 => Ok(20),
    3 => Ok(21),
    _ => Err(0)
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    print_help("Exactly 2 arguments expected");
    process::exit(1);
  }

  let relay = &args[1];
  let relay: u8 = match relay.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      print_help("Invalid Pin Argument");
      process::exit(2);
    },
  };

  let seconds = &args[2];
  let seconds: u64 = match seconds.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      print_help("Invalid Seconds Argument");
      process::exit(3);
    },
  };

  let pin_number: u8 = match pins(relay) {
    Ok(num) => num,
    Err(_) => { 
      print_help("Invalid Relay");
      process::exit(4);
    },
  };

  let mut pin = Gpio::new()?.get(pin_number)?.into_output();

  pin.set_high();
  thread::sleep(Duration::new(seconds, 0));
  pin.set_low();
  println!("Activating relay {} for {} seconds.", relay, seconds);
  Ok(())
}
