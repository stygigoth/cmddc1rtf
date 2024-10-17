use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiIO, MidiInput, MidiOutput, MidiOutputConnection};

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(not(target_arch = "wasm32"))] //conn_out is not 'Send' in Web MIDI
fn run() -> Result<(), Box<dyn Error>> {
    use std::cmp::{max, min};

    let mut rotaries = HashMap::new();
    rotaries.insert(32, 0);
    for i in 16..24 {
        rotaries.insert(i, 0);
    }

    let mut midi_in = MidiInput::new("midir forwarding input")?;
    midi_in.ignore(Ignore::None);

    let in_port = select_port(&midi_in, "input")?;
    println!();
    let mut conn_out = get_out()?;

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(&in_port)?;

    let _conn_in = midi_in.connect(
        &in_port,
        "midir-forward",
        move |_, message, _| {
            let mut new = message[2] as isize;
            if rotaries.contains_key(&message[1]) && (message[2] != 0) && (message[2] != 127) {
                new = *rotaries.get(&message[1]).unwrap() as isize
                    + match message[2] {
                        63 => -1,
                        65 => 1,
                        _ => 0,
                    };
                new = max(0, min(127, new));
                rotaries.insert(message[1], new as u8);
            }
            conn_out
                .send(&[message[0], message[1], new as u8])
                .unwrap_or_else(|_| println!("Error when forwarding message..."));
        },
        (),
    )?;

    println!(
        "Connection open, forwarding from '{}' (press enter to exit)...",
        in_port_name
    );

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    println!("Closing connections");
    Ok(())
}

fn select_port<T: MidiIO>(midi_io: &T, descr: &str) -> Result<T::Port, Box<dyn Error>> {
    println!("Available {} ports:", descr);
    let midi_ports = midi_io.ports();
    for (i, p) in midi_ports.iter().enumerate() {
        println!("{}: {}", i, midi_io.port_name(p)?);
    }
    print!("Please select {} port: ", descr);
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let port = midi_ports
        .get(input.trim().parse::<usize>()?)
        .ok_or("Invalid port number")?;
    Ok(port.clone())
}

#[cfg(unix)]
fn get_out() -> Result<MidiOutputConnection, Box<dyn Error>> {
    use midir::os::unix::VirtualOutput;

    let connection = MidiOutput::create_virtual(MidiOutput::new("CMDDC1RTF")?, "Virtual Output")?;
    Ok(connection)
}

#[cfg(not(unix))]
fn get_out() -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("CMD DC-1 Virtual Output")?;
    let out_port = select_port(&midi_out, "output")?;
    let connection = midi_out.connect(&out_port, "midir-forward")?;

    Ok(connection)
}

#[cfg(target_arch = "wasm32")]
fn run() -> Result<(), Box<dyn Error>> {
    println!("This program cannot run on Web MIDI");
    Ok(())
}
