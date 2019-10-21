mod bus;
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::io::{stdout, Write};

fn draw_screen(pixels: &[u8]) {
    let width = 40;
    let height = 24;
    let depth = 1;
    let bytes_per_row = width / (8 / depth) + if width % 8 != 0 {1} else {0};

    if pixels.len() < bytes_per_row * height {
        panic!("not enough pixels");
    }

    let _pixel_state_list = [
        " ",
        "\u{2580}",
        "\u{2584}",
        "\u{2588}",
    ];


    let pixel_state_list = [
        "\u{2591}\u{2591}", // Light Shade
        "\u{2593}\u{2593}", // Heavy Shade
    ];

    let _long_pixel_state_list = [
        "\u{2591}\u{2591}", // Light Shade
        "\u{2593}\u{2593}", // Heavy Shade
        " \\\u{20D8}", // Combining Ring Overlay
        " /\u{20D8}", // Combining Ring Overlay
        " .",
        "[]",
        "\u{2591}\u{2591}", // Light Shade
        "\u{2592}\u{2592}", // Medium Shade
        "\u{2593}\u{2593}", // Heavy Shade
        "\u{2588}\u{2588}", // Full Block
        "<!",
        "!>",
        "\\/",
        "::",
        "__",
        "==",
    ];

    for y_index in 0..height {
        if y_index != 0 { print!("\r\n"); }
        //~ let line = String::new();
        for row_byte_index in 0..bytes_per_row {
            for bit_index in 0..=7 {
                let pixel: u8 = pixels[y_index * bytes_per_row + row_byte_index];
                //~ let bit = pixel & (1 << (7 - bit_index));
                let bit = (pixel & (1 << (7 - bit_index))) >> (7 - bit_index);
                //~ let bit1 = if y_index > 8 {0} else {1};
                //~ let bit2 = if row_byte_index >= 2 {0} else {1};
                let bit1 = 0;
                let bit2 = 0;
                let pixel_state = pixel_state_list[bit as usize | (bit1 << 1) | (bit2 << 2)];
                print!("{}", pixel_state);
            }
        }
    }

}

struct VvEmulator {
    bus_devices: Vec<Box<dyn bus::BusDevice>>,
}

impl VvEmulator {
    //~ pub fn new() -> Self {
        //~ VvEmulator {
            //~ bus_devices: vec![],
        //~ }
    //~ }

    fn read(&mut self, address: u16) -> u8 {
        for device in self.bus_devices.iter_mut().rev() {
            if device.address_in_range(address) {
                return device.read(address)
            }
        }
        eprintln!("Open Bus read at {:04x}.", address);
        (address >> 8) as u8
    }

    fn write(&mut self, address: u16, byte: u8) -> () {
        for device in self.bus_devices.iter_mut().rev() {
            if device.address_in_range(address) {
                return device.write(address, byte)
            }
        }
        eprintln!("Open Bus write at {:04x} of {:02x}.", address, byte);
    }
}

//~ // TODO: add basic argument parsing to load rom at runtime
//~ use std::env;

fn load_file(filename: &str) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;
    let mut f = File::open(filename).expect("file not found");
    let mut file_data = vec![];
    f.read_to_end(&mut file_data)
        .expect("something went wrong reading the file");
    file_data
}

fn poke_screen_bit(pixels: &mut [u8], x: usize, y: usize, bit: u8) {
    let bit_index = (7 - x % 8) as u8;
    let byte_index = y * 5 + (x / 8);
    let byte = pixels[byte_index];
    pixels[byte_index] = (byte | (bit << bit_index)) & !((0x1 ^ bit) << bit_index);
}

fn peek_screen_bit(pixels: &mut [u8], x: usize, y: usize) -> u8 {
    let bit_index = (7 - x % 8) as u8;
    let byte_index = y * 5 + (x / 8);
    let byte = pixels[byte_index];
    (byte & (1 << bit_index)) >> bit_index
}

fn main() {
    match std::panic::catch_unwind(|| {
        other_main()
    }) {
        Ok(_) => (),
        Err(e) => {
            // do stuff
            eprintln!("{:?}", e);
            ()
        },
    }
}

fn other_main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = load_file("rom.vvp");

    let open_bus = bus::OpenBus {};

    let rom_controller = bus::RomController::from_bytes(rom);

    let mut bus_devices: Vec<Box<dyn bus::BusDevice>> = vec![];

    bus_devices.push(Box::new(open_bus));
    bus_devices.push(Box::new(rom_controller));

    let mut vve = VvEmulator {
        bus_devices
    };

    /*
    for i in 0..=63 {
        let addr = i + 0x3fe0;
        //~ dbg!(rom_controller.read(i + 0x3ff0));
        vve.write(addr, 0);
        let _byte_read = vve.read(addr);
        dbg!(byte_read);
    }
    */

    let nanoseconds_in_60_hz = 16666666;
    let target_frame_duration = Duration::new(0, nanoseconds_in_60_hz);
    //~ let target_frame_duration = Duration::new(1, 0);

    let mut frame_duration;
    let mut start_instant = Instant::now();

    let mut pixel_buffer = [0u8; 40 / 8 * 24];

    //~ for i in 0..40 / 8 * 24 {
        //~ pixel_buffer[i] = (i % 256) as u8;
    //~ }

    //*
    for byte in 0..5 {
        for row in 0..24 {
            pixel_buffer[row * 5 + byte] = if row == 0 || row == 23 {
                0xff
            } else {
                if byte == 0 {
                    0x80
                } else if byte == 4 {
                    0x01
                } else {
                    0x00
                }
            };
        }
    }
    // */


    use crossterm::{Crossterm, InputEvent, KeyEvent};

    let crossterm_instance = Crossterm::new();
    let screen = crossterm::AlternateScreen::to_alternate(true)?;
    let input = crossterm_instance.input();
    let mut stdin = input.read_async();
    let cursor = crossterm::cursor();

    cursor.hide()?;

    cursor.goto(0, 0)?;

    let terminal = crossterm::terminal();
    terminal.clear(crossterm::ClearType::All)?;

    // TODO: Add a check for if the terminal has changed size to have dynamic size
    //~ let terminal_size = terminal.size()?;

    let mut hero = (20, 12);

    'main: loop {
        let mut controller1 = 0u8;
        while let Some(b) = stdin.next() {
            match b {
                InputEvent::Keyboard(event) => match event {
                    KeyEvent::Char('q') => break 'main,
                    KeyEvent::Char('c') => controller1 = controller1 | (1 << 7),
                    KeyEvent::Char('x') => controller1 = controller1 | (1 << 6),
                    KeyEvent::Char('z') => controller1 = controller1 | (1 << 5),
                    KeyEvent::Enter => controller1 = controller1 | (1 << 4),
                    KeyEvent::Up => controller1 = controller1 | (1 << 3),
                    KeyEvent::Down => controller1 = controller1 | (1 << 2),
                    KeyEvent::Left => controller1 = controller1 | (1 << 1),
                    KeyEvent::Right => controller1 = controller1 | (1 << 0),
                    _ => {}
                },
                _ => {}
            }
        }

        // do something with controller1 input to make hero_coords and the associated pixel move
        poke_screen_bit(&mut pixel_buffer, hero.0, hero.1, 0);

        let directional = controller1 & 0xf;

        let delta = (
            ((directional & (1 << 0)) >> 0) as isize - ((directional & (1 << 1)) >> 1) as isize,
            ((directional & (1 << 2)) >> 2) as isize - ((directional & (1 << 3)) >> 3) as isize,
        );

        let new_loc = ((hero.0 as isize + delta.0) as usize, (hero.1 as isize + delta.1) as usize);

        //~ if peek_screen_bit(&mut pixel_buffer, new_loc.0, new_loc.1) == 0 {
        //~ if new_loc.0 < 40 && new_loc.1 < 24 {
        if (new_loc.0 < 40 && new_loc.1 < 24) && peek_screen_bit(&mut pixel_buffer, new_loc.0, new_loc.1) == 0 {
            hero = new_loc;
        }

        poke_screen_bit(&mut pixel_buffer, hero.0, hero.1, 1);
        //~ pixel_buffer[11] = controller1;

        cursor.goto(0, 0)?;

        draw_screen(&pixel_buffer);

        stdout().flush()?;

        frame_duration = Instant::now().duration_since(start_instant);
        start_instant = Instant::now();
        if frame_duration < target_frame_duration {
            sleep(target_frame_duration - frame_duration);
        }
    }

    cursor.show()?;
    screen.to_main()?;
    Ok(())
}
