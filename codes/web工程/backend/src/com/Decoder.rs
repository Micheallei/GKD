
//extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::fs;
use std::convert::TryInto;


const BYTES_IN_INT:i32 = 4;
pub struct Decoder{

}

impl Decoder{
    pub fn decode(shardsFolder:PathBuf,fileFolder:PathBuf,fid:i32,noa:i32) -> bool{
        /*！  未处理throw IOException  */ 

        let totalShards:i32 = noa;
        let dataShards:i32 = noa / 2;

        // Read in any of the shards that are present.
        // (There should be checking here to make sure the input
        // shards are the same size, but there isn't.)
        //let mut shards: Vec<Vec<u8>>;   //byte [totalShards] []
    
        let mut shards:Vec<Vec<u8>> = vec![Vec::new();totalShards.try_into().unwrap()];
        let mut shardPresent:Vec<bool> = Vec::new();
        //let mut shardPresent:[bool;totalShards];//boolean [totalShards]
        let mut shardSize = 0;
        let mut shardCount = 0;
        let i:i32 = 0;
        for i in 0..totalShards as usize{
            let pathbuf = shardsFolder.join(Path::new(&(fid * 100 +i as i32).to_string()));
            let shardFile_path:&Path = pathbuf.as_path();
            if shardFile_path.exists() {
                let mut shardFile:File = File::open(&shardFile_path).unwrap();
                shardSize = shardFile.metadata().unwrap().len(); //len()返回u64
                shardPresent.push(true);
                shardCount += 1;
                &shards[i].append(&mut fs::read(&shardFile_path).unwrap());
            }else {
                shardPresent.push(false);
            }
            
        }

        // We need at least dataShards to be able to reconstruct the file.
        if shardCount < dataShards {
            println!("Not enough shards present");
            return false;
        }

        // Make empty buffers for the missing shards.

        let mut shards:Vec<_> = shards.iter().cloned().map(Some).collect();

        for i in 0..totalShards as usize {
            if !shardPresent[i] {
                // for j in 0..shardSize {
                //     &shards[i].push(0);
                // }
                shards[i] = None;
            }
        }


        // Use Reed-Solomon to fill in the missing shards
        let mut reedSolomon = ReedSolomon::new(dataShards.try_into().unwrap(),(totalShards - dataShards).try_into().unwrap()).unwrap();
        //reedSolomon.reconstruct(shards).unwrap();
        reedSolomon.reconstruct(&mut shards).unwrap();

        // Combine the data shards into one buffer for convenience.
        // (This is not efficient, but it is convenient.)
        let mut allBytes:Vec<u8> = Vec::new();
        for i in 0..dataShards as usize {
            for j in 0..shardSize as usize {
                //allBytes.push(shards[i * (shardSize as usize) + j as usize].unwrap());
                allBytes.push((shards[i].as_ref().unwrap())[j]);
            }
        }

        // Extract the file length
        //filesize

        // Write the decoded file
        //println!("{}",fileFolder.display());
        //let pathbuf = fileFolder.join(Path::new(&(fid).to_string()));
        //let decodedFile_path:&Path = fileFolder.as_path();
        allBytes.remove(0);
        allBytes.remove(0);
        allBytes.remove(0);
        allBytes.remove(0);
        //println!("{}",decodedFile_path.display());
        fs::write(fileFolder,allBytes).unwrap();
        
        println!("Decode Success");
        return true;
    }
}