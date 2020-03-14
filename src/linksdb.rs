extern crate memmap;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::fs::File;
use memmap::MmapMut;

mod sbt_link;
//mod sbt_methods;

pub use sbt_link::Link;




struct LinksDB<T> {
    links: *mut Link<T>,
    allocated_links: T,
    reserved_links: T,
    free_links: T,
    first_free_link: T,
    last_free_link: T,
    block_size: T,
    db_file: File,
    mmap: MmapMut
}

impl<T> LinksDB<T> {
    fn open(&mut self, path: PathBuf) {
        self.db_file = OpenOptions::new()
                       .read(true)
                       .write(true)
                       .create(true)
                       .open(&path)
                       .ok()
                       .unwrap();
        unsafe {
            self.mmap = MmapMut::map_mut(&self.db_file)
                .ok()
                .unwrap();
            let mmap_ptr = self.mmap
                .first()
                .unwrap();

            self.links = std::mem::transmute::<&u8, *mut Link<T>>(mmap_ptr);
        }; 
    }
}

