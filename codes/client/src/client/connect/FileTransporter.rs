use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;

pub fn recv_file(mut f: File, mut soc_in: &TcpStream)->bool{
    println!("enter FileTransporter -- recv_file");
    //原java文件中socout这个参数并没有用到，此处删去
    //手动实现读取一个long类型的数据
    let mut buffer = [0; 8];
    let size = soc_in.read_exact(&mut buffer);
    if let Err(e) = size {
        return false;
    }
    //Java 数据传输都是big endian，此处也默认读到数据是big endian
    //from_bytes is a nightly-only experimental API.
    //let file_length = i64::from_bytes(buffer);

    
      let file_length:i64 = ((buffer[0] as i64) << 56) + (((buffer[1] as i64) & 255) << 48) + (((buffer[2] as i64) & 255) << 40)       
      + (((buffer[3] as i64) & 255) << 32) + (((buffer[4] as i64) & 255) << 24) + (((buffer[5] as i64) & 255) << 16)        
      + (((buffer[6] as i64) & 255) << 8) + (((buffer[7] as i64) & 255) << 0);

    let mut toread:i64 = file_length;
    let mut send_bytes = [0; 1024];

    println!("file_length:{}",file_length);


    while toread > 1024{
        //soc_in.read_exact(&mut send_bytes).unwrap();
        //toread = toread - 1024;
        soc_in.read_exact(&mut send_bytes).unwrap();
        toread = toread - 1024;

        f.write(&send_bytes[..]).unwrap();//test
        f.flush();
    }

    let readlen = soc_in.read(&mut send_bytes).unwrap();
    f.write(&send_bytes[0..readlen]).unwrap();
    f.flush();
    //let mut file_end: Vec<u8> = Vec::new();
    //println!("before read_to_end");
    //soc_in.read_to_end(&mut file_end).unwrap();
    //println!("after read_to_end");
    //f.write(&file_end).unwrap();//test
    //f.flush();
    println!("recv_file success");
    //没有再创建FileOutputStream对象，这里不需要关闭什么
    return true
}//TODO:err handle

pub fn send_file(mut f: File, mut soc_out: &TcpStream)->bool{
    println!("enter connect-send_file");
    let mut send_bytes = [0; 1024];

    let length = f.metadata().unwrap().len();

    let send_length = soc_out.write(&length.to_be_bytes()).unwrap();
    soc_out.flush();
    //test
    println!("send_file--发送的length应为8bytes，实际发送:{}bytes",send_length);
    //println!("filelength:{}",length);

    let mut file_toread = length;

    while file_toread > 1024 {
        f.read_exact(&mut send_bytes[..]).unwrap();//还未写错误处理
        file_toread = file_toread - 1024;
        soc_out.write(&mut send_bytes[..]);
        soc_out.flush();
    }

    let readlen = f.read(&mut send_bytes[..]);

    let len: i32 = match readlen{
        Err(e) => -1,
        Ok(len) => len as i32,
    };
    if len == -1 {
        return false;
    }

    if len == 0 {
        return true;
    }

    soc_out.write(&mut send_bytes[0..len as usize]);
    // loop {
    //     let readlen = f.read(&mut send_bytes[..]);
    //     let len: i32 = match readlen{
    //         Err(e) => -1,
    //         Ok(len) => len as i32,
    //     };
        
    //     if len == -1 {
    //         return false;
    //     }
    //     if len == 0 {
    //         break;
    //     }
    //     soc_out.write(&mut send_bytes[0..length as usize]);
    //     //let eof = [-1;1];
    //     //soc_out.write(&eof);
    //     soc_out.flush();
    //     println!("send_bytes:");
    //     // for i in 0..length as usize{
    //     //     println!("{}",send_bytes[i]);
    //     // }
    // }
     println!("end connect-send_file");
    return true
}