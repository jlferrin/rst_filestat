// use std::error::Error;


fn unixepoch2date(segundos: u64) -> String {
    extern crate chrono;
    use chrono::prelude::DateTime;
    use chrono::{Local};
    use std::time::{UNIX_EPOCH, Duration};

    let d = UNIX_EPOCH + Duration::from_secs(segundos);
    let datetime = DateTime::<Local>::from(d);

    { datetime.format("%d/%m/%Y %H:%M:%S").to_string() }
    
}

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {

        let filename = args[1].clone();

        Config { filename }
    }
}

pub struct StatFile {
    pub filename: String,
    pub file_type: String,
    pub size: u64,
    pub dev_id: u64,
    pub inode: u64,
    pub mode: u32,
    pub hardlink: u64,
    pub uid: u32,
    pub gid: u32,
    pub rdev_id: u64,
    pub atime: i64,
    pub mtime: i64,
    pub ctime: i64,
    pub blksize: u64,
    pub blocks: u64,
}

impl StatFile {

    pub fn new(file: &String) -> StatFile {
        use std::fs;
        use std::os::unix::fs::FileTypeExt;
        use std::os::unix::fs::MetadataExt;

        let filename = file.clone();

        let metadata = fs::metadata(&filename).expect("No file");
        let ft = metadata.file_type();

        let file_type = if ft.is_file() {
           "File".to_string()
        } else if ft.is_dir() {
           "Directory".to_string()
        } else if ft.is_symlink() {
           "Symlink".to_string()
        } else if ft.is_block_device() {
           "Block device".to_string()
        } else if ft.is_char_device() {
           "Char device".to_string()
        } else if ft.is_fifo() {
           "FIFO".to_string()
        } else if ft.is_socket() {
           "Socket".to_string()
        } else {
           "desconocido".to_string()
        };

        let size = metadata.len();
        let dev_id = metadata.dev();
        let inode = metadata.ino();
        let mode = metadata.mode();

        let hardlink = metadata.nlink();
        let uid = metadata.uid();
        let gid = metadata.gid();
        let rdev_id = metadata.rdev();
        let atime = metadata.atime();
        let mtime = metadata.mtime();
        let ctime = metadata.ctime();
        let blksize = metadata.blksize();
        let blocks = metadata.blocks();
        StatFile { filename, file_type, size, dev_id, inode, mode, hardlink, uid, gid, rdev_id, atime, mtime, ctime, blksize, blocks }
    }

    pub fn print(&self) {
        println!("Fichero.....: {:?}", self.filename);
        println!("Tipo........: {:?}", self.file_type);
        println!("Tamaño......: {:?}", self.size);
        println!("Device......: {:?}", self.dev_id);
        println!("RDevice.....: {:?}", self.rdev_id);
        println!("Inode.......: {:?}", self.inode);
        println!("HLinks......: {:?}", self.hardlink);
        println!("Mode........: {:o}", self.mode);
        println!("UID.........: {:?}", self.uid);
        println!("GID.........: {:?}", self.gid);
        println!("Creación....: {:?}", unixepoch2date(self.ctime as u64));
        println!("Modificación: {:?}", unixepoch2date(self.mtime as u64));
        println!("Acceso......: {:?}", unixepoch2date(self.atime as u64));
        println!("Bloques/tam.: {:?}/{:?}", self.blocks, self.blksize);
    }

    pub fn println(&self) {
        println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:o} {:?} {:?} {:?} {:?} {:?} {:?} {:?} ", self.filename, self.file_type, self.size, self.dev_id, self.rdev_id, self.inode, self.hardlink, self.mode, self.uid, self.gid, unixepoch2date(self.ctime as u64), unixepoch2date(self.mtime as u64), unixepoch2date(self.atime as u64), self.blocks, self.blksize);
    }

}

