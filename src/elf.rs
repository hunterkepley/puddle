use core::slice::to_ptr;

#[packed]
struct ELFIdent {
    ei_mag: [u8, ..4], 
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8, ..7]
}

#[packed]
struct ELFHeader {
    e_ident: ELFIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16
}

pub fn read_header<'a>(contents: &'a [u8]) -> &'a ELFHeader {
    unsafe {
        let x : *ELFHeader = to_ptr(contents) as *ELFHeader;
        //let x : *ELFHeader = contents.as_ptr() as *ELFHeader;
        return &*x;
    }
}

#[test]
fn test_read_elf_from_file() {
    use std::io::File;
    use std::io::{Open, ReadWrite};
    let path = Path::new("build/programs/do-nothing");
    let mut stream = File::open_mode(&path, Open, ReadWrite);
    let bytes = stream.read_to_end();
    let header = read_header(bytes);
    // Check the magic bytes
    assert!(header.e_ident.ei_mag.slice(1,4) == "ELF".as_bytes());
    assert!(header.e_entry == 0x80480b8);
}
