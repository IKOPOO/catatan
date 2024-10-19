use std::{
    cell::{RefCell, RefMut},
    io,
    rc::{Rc, Weak},
};

// struct untuk folder
#[derive(Debug)]
struct Folder {
    name: String,
}

// struct untuk file
#[derive(Debug)]
struct File {
    name: String,
    // size: usize,
}

// enum untuk masuk apakah file atau folder
#[derive(Debug)]
enum FilesystemEntry {
    Folder(Folder),
    File(File),
}

// struct untuk membuat node tree
#[derive(Debug)]
struct TreeNode {
    entry: FilesystemEntry,
    child: Option<Vec<Rc<RefCell<TreeNode>>>>,
    parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    // membuat node tree baru
    fn new_folder(name: String) -> Rc<RefCell<TreeNode>> {
        let folder = Folder { name };
        Rc::new(RefCell::new(TreeNode {
            entry: FilesystemEntry::Folder(folder),
            child: Some(Vec::new()),
            parent: None,
        }))
    }

    // fungsi menambahkan anak dan membuat folder atau file
    fn add_child(parent: &Rc<RefCell<TreeNode>>, name: String, what: String) {
        let mut parent_borrow = parent.borrow_mut();
        if what == "folder" {
            if let FilesystemEntry::Folder(_) = parent.borrow().entry {
                let new_folder = Folder { name };
                let new_node = Rc::new(RefCell::new(TreeNode {
                    entry: FilesystemEntry::Folder(new_folder),
                    child: Some(Vec::new()),
                    parent: Some(Rc::downgrade(parent)),
                }));
                
                if let Some(ref mut children) = parent_borrow.child{
                    children.push(new_node);
                }
            }
        } else if what == "file" {
            if let FilesystemEntry::Folder(_) = parent.borrow().entry {
                let new_file = File { name };
                let new_leaf = Rc::new(RefCell::new(TreeNode {
                    entry: FilesystemEntry::File(new_file),
                    child: None,
                    parent: Some(Rc::downgrade(parent)),
                }));
                
                if let Some(ref mut childre ) = parent_borrow.child{
                    childre.push(new_leaf);
                }
                
            } else {
                println!("blok file ra iso nduwe children anjg tol kontol!!!");
            }
        } else {
            println!("pilihane ra ono blok, kei reti aku cok ben iso jalan")
        }
    }
}

fn garis() {
    let mut a = 0;
    while a < 60 {
        print!("=");
        a += 1;
    }
    println!("");
}
fn single() {
    let mut a = 0;
    while a < 60 {
        print!("_");
        a += 1;
    }
    println!("");
}
fn main() {
    let root = TreeNode::new_folder("root".to_string());
    println!("welcome to .......");
    println!("whatever i don't care what you call this program");
    println!("but rust always like a dick");
    garis();

    println!("we have menu : ");
    println!("[-]catatan \n[-]pengingat");
    single();
    println!("input : ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("inputanmu rak genah su");

    let input = user_input.trim();
    match input {
        "catatan" => {
            println!("Buat Catatan \nLihat catatan\n");
            single();
            println!("input : ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("ra ono cok jo gendeng o koe su");
            let user_input = input.trim();
            match user_input {
                "buat" => {
                    println!("Folder \nFile ");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("ra ono cok jo gendeng o koe su");
                    let user_input = input.trim();
                    match user_input {
                        "folder" => {
                            println!("nama folder : ");
                            let mut nama = String::new();
                            io::stdin().read_line(&mut nama).expect("i love you");
                            let kond = String::from("folder");
                            TreeNode::add_child(&root, nama, kond);
                        }

                        _ => println!("kontrakan"),
                    }
                }
                _ => println!("kontols"),
            }   
        }

        _ => println!("goblok su aku capek anjg rust memek"),
    }
}
