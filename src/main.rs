use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use voicemeeter::VoicemeeterRemote;

fn main() {
    println!("Connecting to VoiceMeeter...");

    let mut remote = VoicemeeterRemote::new().expect("Failed initialize VoiceMeeter API");

    set_vm_type(&mut remote);

    println!("VoiceMeeter connected.");

    // Channel for quitting gracefully
    let (tx, rx) = mpsc::channel();

    // What strips are we controlling?
    let game_strip = 3;
    let chat_strip = 4;

    // Set HID vendor and product IDs
    let (vid, pid) = (0x1038, 0x2258);

    let api = hidapi::HidApi::new().unwrap();

    let hid = thread::spawn(move || {
        let device = api.open(vid, pid).unwrap();
        println!("Listening for ChatMix adjustments... (q to quit)");
        loop {
            // Update parameters
            match remote.is_parameters_dirty() {
                Ok(_x) => {}
                Err(e) => {
                    println!("Could not check parameters. (Error: {})", e);
                    thread::sleep(Duration::from_secs(5));
                    set_vm_type(&mut remote);
                }
            }
            // Read data from device
            let mut buf = [0u8; 8];
            device.read_timeout(&mut buf[..], 500).unwrap();
            if buf[0] == 69 {
                let game_val: f32 = buf[1].into();
                let chat_val: f32 = buf[2].into();
                let new_game_gain = (game_val / 100.00) * 40.00 - 40.00;
                let new_chat_gain = (chat_val / 100.00) * 40.00 - 40.00;
                remote
                    .parameters()
                    .strip(game_strip)
                    .expect("Failed to reach game strip")
                    .gain()
                    .set(new_game_gain)
                    .expect("Failed to set gain");
                remote
                    .parameters()
                    .strip(chat_strip)
                    .expect("Failed to reach chat strip")
                    .gain()
                    .set(new_chat_gain)
                    .expect("Failed to set gain");
            }
            if rx.try_recv().is_ok() {
                println!("Disconnecting from VoiceMeeter...");
                remote.logout().expect("VoiceMeeter Logout Failed.");
                println! {"Disconnected from VoiceMeeter."}
                println!("Stopping HID monitoring...");
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        println!("HID monitoring stopped.");
    });

    // Handle commands (just quit for now)
    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read command");

    let quit_command = "quit";
    if command.trim().to_lowercase() == quit_command
        || command.trim().to_lowercase() == "exit"
        || command.trim().to_lowercase() == "q"
    {
        println!("Quittin' time!");
        let run = 0;
        tx.send(run).unwrap();
        hid.join().unwrap();
        println! {"Exiting now..."}
        std::process::exit(0);
    } else {
        println!("Bad exit! VoiceMeeter Remote not stopped!");
    }
}

fn set_vm_type(remote: &mut VoicemeeterRemote) {
    let voicemeeter_type = loop {
        match remote.get_voicemeeter_type() {
            Ok(t) => break t,
            Err(e) => println!("Error getting VoiceMeeter type. (Error: {}) Is VoiceMeeter running? Retrying in 5s...", e)
        };
        thread::sleep(Duration::from_secs(5));
    };
    println!("Type detected: {}", voicemeeter_type);

    remote.program = voicemeeter_type;
}