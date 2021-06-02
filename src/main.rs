use rppal::gpio::Gpio;
use std::error::Error;
use std::{env, process, thread};
use float_duration::FloatDuration;


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

// GPIO library does not seem to handle clone/copy/move
// so we set the pin up before each operation, instead of
// just setting it up once.
fn pin_on(pin_number: u8) -> Result<(), Box<dyn Error>> {
  let mut pin = Gpio::new()?.get(pin_number)?.into_output();
  pin.set_reset_on_drop(false);
  pin.set_high();
  Ok(())
}

// GPIO library does not seem to handle clone/copy/move
// so we set the pin up before each operation, instead of
// just setting it up once.
fn pin_off(pin_number: u8) -> Result<(), Box<dyn Error>> {
  let mut pin = Gpio::new()?.get(pin_number)?.into_output();
  pin.set_reset_on_drop(false);
  pin.set_low();
  Ok(())
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
  let seconds: f64 = match seconds.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      print_help("Invalid Seconds Argument");
      process::exit(3);
    },
  };
  let dura = FloatDuration::seconds(seconds).to_std()?;

  let pin_number: u8 = match pins(relay) {
    Ok(num) => num,
    Err(_) => { 
      print_help("Invalid Relay");
      process::exit(4);
    },
  };
  

  // The requested GPIO pin will remain in high state if the program is killed
  // during the sleep phase. This will attempt to clean up and set the pin to low.
  let pn = pin_number;
  ctrlc::set_handler(move || {
    pin_off(pn).unwrap();
    process::exit(0);
  }).expect("Error setting up Ctrl-C handler");

  println!("Activating relay {} for {} seconds.", relay, seconds);

  pin_on(pin_number).unwrap(); 
  thread::sleep(dura);
  pin_off(pin_number).unwrap();
  
  Ok(())
}
