
#[macro_use]
extern crate log;

use env_logger;

use std::{
    env,
    fs::File,
    io::{
        Read,
        Write,
    },
    net::{
        TcpListener,
        TcpStream,
    },
    process,
    thread,
};

use cyber_range::emulator::{
    Emulator, 
    Register,
    init_instructions,
};

const MEMORY_SIZE: usize = 0x8000;

fn handler(mut stream: TcpStream, mut emu: Emulator) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    stream.write_all(b"Input the password.\n")?;
    let nbytes = stream.read(&mut buffer)?;
    if nbytes == 0 {
        debug!("Connection closed.");
        return Ok(());
    }
    emu.registers[Register::ESP as usize] -= 0x50;
    emu.registers[Register::EBP as usize] = 0x7c00;
    for i in 0x00..std::cmp::min(0x3f, nbytes-1) as usize { // 最後の文字はLF
        emu.memory[(emu.registers[Register::EBP as usize] as usize - (0x40 - i)) as usize] = buffer[i];
    }
    let instructions = init_instructions();

    while emu.eip < MEMORY_SIZE as u32 {
        let opcode = emu.get_code8(0);
        debug!("EIP = {:0x}, Opcode = {:02x}", emu.eip, opcode);

        if let Some(instruction) = instructions[opcode as usize] {
            instruction(&mut emu);
        } else {
            error!("Not Implemented: {:0x}\n", opcode);
            stream.write_all(format!("Not Implemented: {:0x}\n", opcode).as_bytes())?;
            break;
        }

        if emu.eip == 0x00 {
            info!("end of program.\n");
            if emu.registers[Register::EAX as usize] == 0x01 {
                stream.write_all(b"Congratulations! Flag is ctf{original_emulator_is_good!}\n")?;
            } else {
                stream.write_all(b"Wrong password.\n")?;
            }
            break;
        }
    }

    debug!("Connection closed.");
    Ok(())
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Bad nunber of arguments. [filename]");
        process::exit(1);
    }

    let mut emu = Emulator::create(MEMORY_SIZE, 0x7c00, 0x7c00);
    let filename = &args[1];
    if let Ok(f) = File::open(filename) {
        f.bytes().enumerate().for_each(|(i, byte)| {
            emu.memory[i + 0x7c00] = byte.unwrap();
        });
    } else {
        error!("Failed to open {}.", filename);
        process::exit(1);
    }

    let listener = TcpListener::bind("127.0.0.1:33333").unwrap();
    loop {
        let (stream, _) = listener.accept().unwrap();
        let emu_clone = emu.clone();
        thread::spawn(move || {
            handler(stream, emu_clone).unwrap_or_else(|error| error!("{}", error));
        });
    }
}


