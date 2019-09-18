use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::sync::{Arc, Mutex};

use libc::{ENOENT, EACCES};
use time::Timespec;

use fuse::{
    Filesystem, FileType, Request, FileAttr,
    ReplyAttr, ReplyDirectory, ReplyEntry, ReplyCreate, ReplyWrite, ReplyData,
};

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

#[derive(Debug, Clone)]
pub enum NodeKind {
    Root, Dir, File,
}

#[derive(Debug, Clone)]
pub struct KotoNode {
    // if parent is None, it'a a root.
    pub parent: Option<Arc<Mutex<KotoNode>>>,
    pub inode: u64,
    pub kind: NodeKind,
    pub children: Vec<Arc<Mutex<KotoNode>>>,
    pub name: String,
    pub data: Vec<u8>,
    pub attr: FileAttr,
}

#[derive(Debug)]
pub struct KotoFS {
    pub root: Arc<Mutex<KotoNode>>,
    pub inodes: HashMap<u64, Arc<Mutex<KotoNode>>>,
}

fn create_file(ino: u64, size: u64, ftype: FileType) -> FileAttr {
    let t = time::now().to_timespec();
    FileAttr {
        ino: ino, size: size, blocks: 0,
        atime: t, mtime: t, ctime: t, crtime: t,
        kind: ftype,
        perm: match ftype {
            FileType::Directory => 0o775,
            _ => 0o644,
        },
        nlink: 2, uid: 501, gid: 20, rdev: 0, flags: 0,
    }
}

fn create_dir(ino: u64) -> FileAttr {
    let t = time::now().to_timespec();
    FileAttr {
        ino: ino, size: 0, blocks: 0,
        atime: t, mtime: t, ctime: t, crtime: t,
        kind: FileType::Directory,
        perm: 0o755,
        nlink: 2, uid: 501, gid: 20, rdev: 0, flags: 0,
    }
}

impl KotoFS {
    pub fn init() -> KotoFS {
        let root = KotoNode {
            inode: 1, kind: NodeKind::Root, children: [].to_vec(), name: "/".to_string(), data: [].to_vec(),
            parent: None, attr: create_file(1, 0, FileType::Directory),
        };
        let root_arc = Arc::new(Mutex::new(root));
        let mut inodes = HashMap::new();
        inodes.insert(root_arc.lock().unwrap().inode, root_arc.clone());
        KotoFS { inodes: inodes, root: root_arc }
    }

    pub fn mount(self, mountpoint: OsString) {
        println!("{:?}", self);
        fuse::mount(self, &mountpoint, &[]).expect(&format!("fail mount() with {:?}", mountpoint));
    }
}

impl Filesystem for KotoFS {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr() with {:?}", ino);
        if let None = self.inodes.get(&ino) {
            reply.error(ENOENT);
            return;
        }

        for (_, n) in self.inodes.iter() {
            if ino == n.lock().unwrap().inode {
                reply.attr(&TTL, &n.lock().unwrap().attr);
                return;
            }
        }

        reply.error(ENOENT);
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        println!("readdir() from {:?}", ino);
        if offset > 0 {
            reply.ok();
            return;
        }
        reply.add(1, 0, FileType::Directory, ".");
        reply.add(2, 1, FileType::Directory, "..");
        let mut reply_add_offset = 2;
        for (_, n) in self.inodes.iter() {
            let attr = n.lock().unwrap().attr;
            let name = n.lock().unwrap().name.to_string();
            if let Some(parent_inode) = &n.lock().unwrap().parent {
                if parent_inode.lock().unwrap().inode == ino {
                    reply.add(attr.ino, reply_add_offset, attr.kind, name);
                    reply_add_offset += 1;
                }
            }
        }
        reply.ok();
    }

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("lookup() by {:?}", name);
        for (_, n) in self.inodes.iter() {
            let node_attr = n.lock().unwrap().attr;
            let node_name = n.lock().unwrap().name.to_string();
            if let Some(parent_node) = &n.lock().unwrap().parent {
                let inode = parent_node.lock().unwrap().inode;
                if inode == parent && name.to_str().unwrap() == node_name {
                    reply.entry(&TTL, &node_attr, 0);
                    return;
                }
            }
        }
        reply.error(ENOENT);
    }

    fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, _flag: u32, reply: ReplyCreate) {
        println!("create() with {:?}", name);
        let inode = time::now().to_timespec().sec as u64;
        let f = create_file(inode, 0, FileType::RegularFile);
        if let Some(parent_node) = self.inodes.get(&parent) {
            let node = KotoNode {
                parent: Some(parent_node.clone()), inode: inode, kind: NodeKind::File, children: Vec::new(),
                name: name.to_str().unwrap().to_string(), data: [].to_vec(), attr: f,
            };
            let node = Arc::new(Mutex::new(node));
            parent_node.lock().unwrap().children.push(node.clone());
            self.inodes.insert(inode, node);
            reply.created(&TTL, &f, 0, 0, 0,);
        }
    }

    fn setattr(&mut self, _req: &Request, ino: u64, _mode: Option<u32>, _uid: Option<u32>, _gid: Option<u32>,
        _size: Option<u64>, _atime: Option<Timespec>, _mtime: Option<Timespec>, _fd: Option<u64>,
        _crtime: Option<Timespec>, _chgtime: Option<Timespec>, _bkuptime: Option<Timespec>, _flags: Option<u32>,
        reply: ReplyAttr) {
        println!("setattr() with {:?}", ino);
        match self.inodes.get(&ino) {
            Some(n) => reply.attr(&TTL, &n.lock().unwrap().attr),
            None => reply.error(EACCES),
        }
    }

    fn write(&mut self, _req: &Request, ino: u64, _fh: u64, _offset: i64, data: &[u8], _flags: u32, reply: ReplyWrite) {
        println!("write() to {:?}", ino);
        let length: usize = data.len();
        if let Some(n) = self.inodes.get_mut(&ino) {
            //let parent_ino = n.lock().unwrap().inode;
            n.lock().unwrap().attr.size = data.len() as u64;
            n.lock().unwrap().data = data.to_vec();
        }
        reply.written(length as u32);
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, _offset: i64, _size: u32, reply: ReplyData) {
        println!("read() from {:?}", ino);
        match self.inodes.get(&ino) {
            Some(n) => {
                let data = &n.lock().unwrap().data;
                println!("{:?}", data);
                reply.data(&data);
            },
            None => reply.error(EACCES),
        }
    }
}
