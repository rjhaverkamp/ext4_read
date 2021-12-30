use byteorder::{LittleEndian, ReadBytesExt};

use std::fs::{File, OpenOptions};

use std::io::prelude::*;
use std::io::{SeekFrom};

use std::path::Path;

#[derive(Debug, Default)]
struct Superblock {
    magic: u16,
    block_size: u32,
    blocks_per_group: u32,
    inodes_per_group: u32,
    inode_size: u32,
}

impl Superblock {
    fn offset_from_start(offset: u64) -> SeekFrom {
        SeekFrom::Start(1024 + offset)
    }
    fn load(f: &mut File) -> Self {
        f.seek(Superblock::offset_from_start(0x18)).unwrap();
        let block_size = f.read_u32::<LittleEndian>().unwrap();
        let block_size = 2u32.pow(10 + block_size);
        f.seek(Superblock::offset_from_start(0x20)).unwrap();
        let blocks_per_group = f.read_u32::<LittleEndian>().unwrap();
        f.seek(Superblock::offset_from_start(0x28)).unwrap();
        let inodes_per_group = f.read_u32::<LittleEndian>().unwrap();
        f.seek(Superblock::offset_from_start(0x38)).unwrap();
        let magic = f.read_u16::<LittleEndian>().unwrap();
        f.seek(Superblock::offset_from_start(0x58)).unwrap();
        let inode_size = f.read_u32::<LittleEndian>().unwrap();

         Superblock {
            block_size,
            blocks_per_group,
            inodes_per_group,
            magic,
            inode_size,
        }
    }
}

fn main() {
    let mut f = OpenOptions::new()
        .read(true)
        .open(Path::new("/dev/loop10"))
        .unwrap();
    let sb = Superblock::load(&mut f);
    assert_eq!(sb.magic, 61267);
    assert_eq!(sb.block_size, 4096);
    assert_eq!(sb.blocks_per_group, 32768);
    assert_eq!(sb.inodes_per_group, 1024);
    assert_eq!(sb.inode_size, 128);
    print!("{:#?}", sb);
}
