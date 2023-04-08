use std::fs;
use std::fs::DirEntry;
use std::io::Write;

#[derive(serde::Deserialize, serde::Serialize)]
struct Manifest {
    pinned_projects: Vec<String>,
    pinned_posts: Vec<String>,
    recent_posts: Vec<String>,
    all: Vec<String>
}

fn main() {
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

    let manifest_string = fs::read_to_string(
        "posts/manifest.yaml".to_string()).unwrap();
    let mut manifest_content = serde_yaml::from_str::<Manifest>(&manifest_string).unwrap();
    let all: Vec<String> = post_list
        .iter()
        .filter(|entry| !entry.is_ok())
        .map(|post_entry| post_entry.dir_name.clone())
        .collect();
    manifest_content.all = all;
    let manifest_write = serde_yaml::to_string::<Manifest>(&manifest_content).unwrap();

    fs::write("posts/manifest.yaml", manifest_write).expect("Could not write manifest");
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
        self.content && self.ok_meta
    }
}

#[derive(serde::Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub tagline: String,
    pub date: String,
    pub tags: Vec<String>,
    pub thumbnails: Vec<String>,
    pub splash_image: String,
}

fn loop_projects(post_entry: DirEntry) -> PostEntry {
    let mut entry = PostEntry::new(post_entry.path().as_os_str().to_str().unwrap().to_string());
    let html_path = format!("{}/content.html", entry.dir_name);
    let post_entry = fs::read_dir(post_entry.path()).unwrap();

    for candidate in post_entry {
        if let Ok(file) = candidate {
            if file.file_name() == "content.md" {
            
                let html_content = markdown::to_html(
                    &fs::read_to_string(
                        file.path().into_os_string()
                    )
                    .unwrap());
                let html_content = add_server_prefix(html_content, entry.dir_name.clone());
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

static IMG_SERVER_PREFIX: &str = "https://orca-app-8uzme.ondigitalocean.app";

fn recursion(html_string: String, mut html_bits: Vec<String>, path_prefix: String) -> Vec<String> {
    match html_string.find("src=") {
        Some(index) => {
            let (one, two) = html_string.split_at(index + 5usize);
            let mut one = one.to_string();
            one.push_str(&path_prefix);
            html_bits.push(one);
            html_bits = recursion(two.to_string(), html_bits, path_prefix);
        }
        None => html_bits.push(html_string),
    };

    html_bits
}

fn add_server_prefix(html_string: String, dir_name: String) -> String {
    let path_prefix = format!("{}/{}/", IMG_SERVER_PREFIX, dir_name);
    let mut html_bits: Vec<String> = Vec::new();
    html_bits = recursion(html_string, html_bits, path_prefix);
    let mut html_final = String::new();
    for bit in html_bits {
        html_final.push_str(&bit);
    }

    html_final
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