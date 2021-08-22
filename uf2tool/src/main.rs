use std::io::Write;

// const UF2_FLAG_IGNORE: u32 = 0x00000001;
// const UF2_FLAG_CONTAINER: u32 = 0x00001000;
const UF2_FLAG_FAMILY_ID: u32 = 0x00002000;
// const UF2_FLAG_MD5: u32 = 0x00004000;
// const UF2_FLAG_EXTENSION: u32 = 0x00008000;
const UF2_MAGIC0: u32 = 0x0A324655;
const UF2_MAGIC1: u32 = 0x9E5D5157;
const UF2_MAGIC2: u32 = 0x0AB16F30;

#[cfg(target_os = "macos")]
static MOUNT_PATH: &str = "/Volumes";
#[cfg(target_os = "linux")]
static MOUNT_PATH: &str = "/media";

#[repr(C)]
struct UF2Block {
    magic0: u32,
    magic1: u32,
    flags: u32,
    target_addr: u32,
    payload_size: u32,
    block_index: u32,
    number_of_blocks: u32,
    family_id: u32,
    payload: [u8; 476],
    magic2: u32,
}

fn write_uf2(output_path: &std::path::Path, binary: &[u8], vaddr: u32, payload_block_size: usize, family_id: u32) {
    let mut output_bytes = std::fs::File::create(output_path).unwrap();
    let number_of_blocks = (binary.len() + payload_block_size - 1) / payload_block_size;
    for block_index in 0..number_of_blocks {
        let mut block = UF2Block {
            magic0: UF2_MAGIC0,
            magic1: UF2_MAGIC1,
            flags: UF2_FLAG_FAMILY_ID,
            target_addr: vaddr + block_index as u32 * payload_block_size as u32,
            payload_size: payload_block_size as u32,
            block_index: block_index as u32,
            number_of_blocks: number_of_blocks as u32,
            family_id,
            payload: [0; 476],
            magic2: UF2_MAGIC2,
        };
        let src_offset = block_index * payload_block_size;
        let src_end_offset = (src_offset + payload_block_size).min(binary.len());
        block.payload[0..src_end_offset - src_offset].copy_from_slice(&binary[src_offset..src_end_offset]);
        output_bytes.write_all(unsafe { &std::mem::transmute::<_, [u8; 512]>(block) }).unwrap();
    }
}

fn main() {
    let matches = clap::App::new("uf2tool")
        .version("0.1.0")
        .author("Alexander Meißner <AlexanderMeissner@gmx.net>")
        .about("Converts executable files to UF2")
        .arg(clap::Arg::new("input")
            .about("File path of input executable")
            .required(true)
            .index(1))
        .arg(clap::Arg::new("output")
            .long("output")
            .value_name("PATH")
            .about("Output UF2 file path")
            .takes_value(true)
            .default_value("out.uf2"))
        .arg(clap::Arg::new("deploy")
            .long("deploy")
            .about("Uploads to device")
            .conflicts_with("output"))
        .arg(clap::Arg::new("block_size")
            .long("block-size")
            .value_name("bytes")
            .about("Number of payload bytes per block")
            .takes_value(true)
            .default_value("256"))
        .arg(clap::Arg::new("family_id")
            .long("family")
            .value_name("HEX")
            .about("Device family id")
            .takes_value(true)
            .default_value("E48BFF56"))
        .get_matches();

    let family_id = u32::from_str_radix(matches.value_of("family_id").unwrap(), 16).unwrap();
    let input_path = std::path::Path::new(matches.value_of("input").unwrap());
    let output_path = if matches.is_present("deploy") {
        match family_id {
            0xE48BFF56 => std::path::Path::new(MOUNT_PATH).join("RPI-RP2/out.uf2"),
            _ => panic!("Unknown family_id for deployment")
        }
    } else {
        std::path::Path::new(matches.value_of("output").unwrap()).to_path_buf()
    };
    let payload_block_size = matches.value_of("block_size").unwrap().parse::<usize>().unwrap();
    let input_bytes = std::fs::read(input_path).unwrap();
    match goblin::Object::parse(&input_bytes).unwrap() {
        goblin::Object::Elf(elf) => {
            if elf.dynamic.is_some() ||
               !elf.dynrelas.is_empty() ||
               !elf.dynrels.is_empty() ||
               !elf.pltrelocs.is_empty() ||
               !elf.libraries.is_empty() {
                panic!("Exectuable uses dynamic linking");
            }
            let mut start_address = 0;
            let mut consecutive_address = None;
            let mut buffer = Vec::new();
            for header in elf.program_headers.iter() {
                if header.p_type == goblin::elf::program_header::PT_LOAD {
                    println!("Load {} bytes at physical={:#08X} virtual={:#08X}", header.p_filesz, header.p_paddr, header.p_vaddr);
                    buffer.extend(input_bytes[header.p_offset as usize..header.p_offset as usize + header.p_filesz as usize].iter().cloned());
                    if let Some(consecutive_address) = consecutive_address {
                        if header.p_paddr as u32 != consecutive_address {
                            panic!("Load address {:#08X} does not continue the previous one {:#08X}", header.p_paddr, consecutive_address);
                        }
                    } else {
                        start_address = header.p_paddr as u32;
                        match family_id {
                            0xE48BFF56 => {
                                if header.p_paddr != 0x10000000 &&
                                   header.p_paddr != 0x15000000 &&
                                   header.p_paddr != 0x20000000 {
                                    panic!("Load address {:#08X} is not supported on this device", header.p_paddr);
                                }
                                if header.p_paddr == 0x10000000 {
                                    let buffer_len = buffer.len();
                                    let mut engine = crc_any::CRCu32::crc32mpeg2();
                                    engine.digest(&buffer[buffer_len - payload_block_size..buffer_len - 4]);
                                    buffer[buffer_len - 4..].clone_from_slice(&engine.get_crc().to_le_bytes());
                                }
                            }
                            _ => {}
                        }
                    }
                    consecutive_address = Some(header.p_paddr as u32 + header.p_filesz as u32);
                }
            }
            if consecutive_address.is_none() {
                panic!("Could not find any load commands");
            }
            write_uf2(
                &output_path,
                &buffer,
                start_address,
                payload_block_size,
                family_id,
            );
        },
        _ => { panic!("Unsupported executable file format") }
    }
}
