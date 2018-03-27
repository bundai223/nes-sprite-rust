use std::{fs, u8};
use std::io::{BufReader, Read};

extern crate bmp;
use bmp::Image;

const NES_HEADER_SIGNATURE_SIZE: usize = 4;
const NES_HEADER_SIZE: usize = 6;
const NES_PROGRAM_ROM_PAGE_SIZE: usize = 16 * 1024 * 1024;
const NES_CHARACTER_ROM_PAGE_SIZE: usize = 8 * 1024 * 1024;

struct Page { start: usize, end: usize, num: usize }
struct Header {
    signature: Vec<char>,
    prg_rom: Page,
    chr_rom: Page
}

fn allocate(size: usize) -> Box<[u8]> {
    let mut tmpvec = Vec::<u8>::with_capacity(size);
    unsafe {
        tmpvec.set_len(size);
    }
    return tmpvec.into_boxed_slice();
}

fn header_dump(rom: Box<[u8]>) -> Header {
    let prg_page = (*rom)[NES_HEADER_SIGNATURE_SIZE + 0] as usize;
    let chr_page = (*rom)[NES_HEADER_SIGNATURE_SIZE + 1] as usize;
    let prg_page_start = NES_HEADER_SIGNATURE_SIZE + 0;
    let prg_page_end   = prg_page_start + prg_page * NES_PROGRAM_ROM_PAGE_SIZE;
    let chr_page_start = prg_page_end;
    let chr_page_end   = chr_page_start + chr_page * NES_CHARACTER_ROM_PAGE_SIZE;

    return Header{
        signature: vec![(*rom)[0].into(), (*rom)[1].into(), (*rom)[2].into(), (*rom)[3].into()],
        prg_rom: Page{ start: prg_page_start, end: prg_page_end, num: prg_page},
        chr_rom: Page{ start: chr_page_start, end: chr_page_end, num: chr_page}
    };
}

fn main() {
    let file = fs::File::open("rom/sample1/sample1.nes").unwrap();
    let filesize = file.metadata().unwrap().len();
    let mut reader = BufReader::new(file);

    let mut rom_data: Box<[u8]> = allocate(filesize as usize);

    reader.read_exact(&mut (*rom_data)).unwrap();

    let header = header_dump(rom_data);

    println!("Header Dump");
    println!("signature: {:?} ", header.signature);
    println!("");
    println!("Size of PRG ROM in 16 KB units : {:X}-{:X}({:X})", header.prg_rom.start, header.prg_rom.end, header.prg_rom.num);
    println!("Size of CHR ROM in  8 KB units : {:X}-{:X}({:X})", header.chr_rom.start, header.chr_rom.end, header.chr_rom.num);


    const NES_SPRITE_DATA_SIZE: usize = 0x10000;
    const DEFAULT_CANVAS_WIDTH: usize = 800;
    const SPRITE_PER_ROW: usize       = DEFAULT_CANVAS_WIDTH / 8;
    let sprite_num: usize           = NES_CHARACTER_ROM_PAGE_SIZE * header.chr_rom.num / NES_SPRITE_DATA_SIZE;
    let row_num: usize              = (sprite_num / SPRITE_PER_ROW) + 1;
    let canvas_height: usize        = row_num * 8;

    println!("canvas dump");
    println!("sprite num : {}({}x{})", sprite_num, SPRITE_PER_ROW, row_num);
    println!("height     : {}", canvas_height);

    // let mut image = Image::new(DEFAULT_CANVAS_WIDTH, canvas_height);
}
