use std::{
    cell::RefCell,
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
struct Filee {
    name: String,
    // size: usize,
}

// enum untuk masuk apakah file atau folder
#[derive(Debug)]
enum FilesystemEntry {
    Folder(Folder),
    File(Filee),
}

// struct untuk membuat node tree
#[derive(Debug)]
struct TreeNode {
    entry: FilesystemEntry,
    child: Option<Vec<Rc<RefCell<TreeNode>>>>,
    parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    // fungsi ini hanya untuk membuat node root/folder root
    fn new_folder(name: String) -> Rc<RefCell<TreeNode>> {
        let folder = Folder { name };
        Rc::new(RefCell::new(TreeNode {
            entry: FilesystemEntry::Folder(folder),
            child: Some(Vec::new()),
            parent: None,
        }))
    }

    // fungsi untuk membuat sebuah folder baru
    fn create_folder(parent: &Rc<RefCell<TreeNode>>, name: String) {
        let new_folder = Folder { name };
        let new_node = Rc::new(RefCell::new(TreeNode {
            entry: FilesystemEntry::Folder(new_folder),
            child: Some(Vec::new()),
            parent: Some(Rc::downgrade(parent)),
        }));

        let mut parent_borrow = parent.borrow_mut();
        if let Some(ref mut children) = parent_borrow.child {
            children.push(new_node);
        }
    }

    // fungsi untuk membuat sebuah file baru
    fn create_file(parent: &Rc<RefCell<TreeNode>>, name: String) {
        let new_file = Filee { name };
        let new_leaf = Rc::new(RefCell::new(TreeNode {
            entry: FilesystemEntry::File(new_file),
            child: None,
            parent: Some(Rc::downgrade(parent)),
        }));

        let mut parent_borrow = parent.borrow_mut();
        if let Some(ref mut children) = parent_borrow.child {
            children.push(new_leaf);
        }
    }
    // fungsi menambahkan anak
    fn add_child(parent: &Rc<RefCell<TreeNode>>, what: String) {
        let mut input = String::new();
        println!("Masukkan nama {}",what);
        io::stdin().read_line(&mut input).expect("cannot read line");
        let name = input.trim().to_string();
        match what.as_str() {
            "folder" => {
                TreeNode::create_folder(parent, name);
            }
            "file" => {
                TreeNode::create_file(parent, name);
            }
            _ => println!("error blog"),
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

            // user input 
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("ra ono cok jo gendeng o koe su");
            let user_input = input.trim();

            // matching user input to pick proses
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
                            TreeNode::add_child(&root, "folder".to_string());
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
