use std::fs;
use std::fs::DirEntry;
use std::io::Write;

fn main() {
    //println!("{}", markdown::to_html("![alt text](lowpoly_2_right_grey.png \"apple\")"));
    let mut post_list: Vec<PostEntry> = Vec::new();


    let posts_dir = fs::read_dir("./posts").unwrap();

    for post_entry in posts_dir {
        let post_entry = post_entry.unwrap();
        if post_entry.file_type().unwrap().is_dir() {
            post_list.push(loop_projects(post_entry));
        }
    }

    for entry in post_list.iter().filter(|entry| entry.is_ok()) {
        println!("{}", entry.dir_name);
    }

    println!("Incomplete post entries:");
    for entry in post_list.iter().filter(|entry| !entry.is_ok()) {
        println!("{}", entry.dir_name);
    }
}

pub struct PostEntry {
    pub dir_name: String,
    pub content: bool,
    pub ok_meta: bool,
}

impl PostEntry {
    pub fn new(dir_name: String) -> Self {
        PostEntry { dir_name: dir_name, content: false, ok_meta: false }
    }

    pub fn is_ok(&self) -> bool {
        self.content & self.ok_meta
    }
}

#[derive(serde::Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub tagline: String,
    pub date: String,
    pub tags: Vec<String>,
    pub images: Vec<String>,
}

fn loop_projects(post_entry: DirEntry) -> PostEntry {
    let mut entry = PostEntry::new(post_entry.path().as_os_str().to_str().unwrap().to_string());
    let html_path = format!("{}/content.html", entry.dir_name);
    let post_entry = fs::read_dir(post_entry.path()).unwrap();

    

    for candidate in post_entry {
        if let Ok(file) = candidate {
            if file.file_name() == "content.md" {
            
                //println!("{:?}", file.path().as_os_str());
                let html_content = markdown::to_html(
                    &fs::read_to_string(
                        file.path().into_os_string()
                    )
                    .unwrap());

                let mut new_file = fs::File::create(&html_path)
                    .expect("Could not create html file");
                new_file.write_all(html_content.as_bytes())
                    .expect("Could not write data to html file");

                entry.content = true;
            } else if file.file_name() == "meta.yaml" {
                let meta_content = fs::read_to_string(file.path().into_os_string()).unwrap();
                match serde_yaml::from_str::<PostMetadata>(&meta_content) {
                    Ok(_) => entry.ok_meta = true,
                    Err(e) => println!("Error reading meta.yaml at {}\nerror: {}", entry.dir_name, e),
                }
            }
        }
    }

    entry
}
// we need a terminal app that writes the metadata based on the content of each directory in /posts
// create list of occupied folders: vec<String>
// for dir in /posts
//  if content.md
//      read as string
//      convert to html
//      save into html file
//      add folder name to list
// save list of posts
// save list of projects