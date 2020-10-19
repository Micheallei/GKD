//use std::fs::File;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::collections::LinkedList;
use std::vec::Vec;
use std::convert::TryInto;
//use std::convert::From::from;

    /**
     * 文件工具类
     */
    pub struct FileUtil{
    }
    
    impl FileUtil{
    
        /**
         * 清空文件夹
         *
         * @param folderPath 文件夹路径
         */
    
        /*由于直接用了path写，所以这段没改写
        public static void clearFolder(String folderPath) {
            clearFolder(new File(folderPath));
        }*/

        // note:因为无法重载，故函数名加上了_str
        /*pub fn clearFolder_str(&self,folderPath:String) {
             self.clearFolder(folderPath.try_into().unwrap());
         }*/
         //note:by lyf string到pathbuf的类型转换尚未实现，但无人调用此方法
    
        /*先用 struct std::path::Path 写，没找到如何从 
        struct::fs::File 得到对应 path 的方法*/ 
        pub fn new() -> FileUtil{
            FileUtil{

            }
        }
        pub fn clearFolder(&self,folder:&PathBuf) {
            println!("enter fileutil--clearFolder");
            let folder:PathBuf = (*folder.clone()).to_path_buf();
            //原代码中folder是 FILE类型
            if folder.is_file() {
                fs::remove_file(&folder);
            } else if folder.is_dir() {
                if let Ok(entries) = fs::read_dir(folder){
                for entry in entries{
                    if let Ok(entry) = entry{
                        if let Ok(metadata) = entry.metadata(){
                            let pathbuf = entry.path();
                            let path:&Path = pathbuf.as_path();
                            if path.is_dir() {
                                self.clearFolder(&path.to_path_buf());
                                fs::remove_dir(path.to_path_buf());
                            } else {
                                //println!("clearfolder:path:{}",path.display());
                                fs::remove_file(path).unwrap();
                            }
                        }
                    
                    }
                }
               }
            }
            
        }
        /**
         * 广度优先遍历文件夹及其子文件夹，获得该文件夹下所有的文件
         *
         * @param folder 顶层文件夹
         * @return 所有的文件
         */
    
         pub fn getAllFiles(folder:&PathBuf) -> LinkedList<PathBuf>{
             /*!原代码中folder是 FILE类型*/
            
            let folder:PathBuf = (*folder.clone()).to_path_buf();
            let mut files:LinkedList<PathBuf> = LinkedList::new();
            let mut queue: LinkedList<PathBuf> = LinkedList::new();          

            queue.push_back(folder.to_path_buf());
    
            while !queue.is_empty() {
                let dir:PathBuf = queue.pop_front().unwrap();
                
                if let Ok(entries) = fs::read_dir(dir){
                for entry in entries{
                    if let Ok(entry) = entry{
                        if let Ok(metadata) = entry.metadata(){
                            
                            let pathbuf = entry.path(); //.path() -> pathbuf类型
                            if pathbuf.is_dir(){
                                queue.push_back(pathbuf);
                            } else {
                                files.push_back(pathbuf);
                            }
                        }
                    
                    }
                }
                }
            }
            files
    
        }
    }
