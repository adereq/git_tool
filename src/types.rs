use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

pub enum Tree {
    BlobEntry { name: String, hash: String},
    TreeEntry { 
        name: String,
        hash: String,
        children: Vec<Tree>,
    }
}

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>,
}

impl Blob {
    pub fn from_path(path: &PathBuf) -> io::Result<Blob> {
        //open file from the path
        let mut file = File::open(path)?;
        //create empty vector
        let mut bytes = Vec::new();
        //read the whole file
        file.read_to_end(&mut bytes)?;

        //create sha object
        let mut sha = Sha1::new();

        sha.input(&bytes);

        Ok(
            Blob {
                hash: sha.result_str(),
                data: bytes,
            }
        )
    }
}