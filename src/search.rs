use std::env::set_current_dir;
use std::path::PathBuf;
use crate::Language;
use walkdir::{WalkDir, DirEntry};
use dirs;

fn finder(lang: Language) -> &'static str {
    match  lang {
        Language::rust => "Cargo.toml",
        Language::python => "requirements.txt",
        Language::java => "pom.xml",
        Language::cpp => "CMakeLists.txt",
        Language::js => "package.json",
    }
}

pub fn find(lang: Language, project: Option<&str>) {
    let paths = dirs::home_dir().unwrap();
    let idef = match lang {
        Language::cpp => Some("ClionProjects"),
        Language::java => Some("IdeaProjects"),
        Language::js => Some("WebstormProjects"),
        Language::python => Some("PycharmProjects"),
        Language::rust => Some("RustroverProjects"),

    };




    let mut serc: Vec<PathBuf> = Vec::new();
    serc.push(paths.join("dev"));
    serc.push(paths.join("AndroidStudioProjects"));
    serc.push(paths.join("projects"));
    serc.push(paths.join("myproject"));
    serc.push(paths.join("code"));
    if let Some(dir) = idef {
        serc.push(paths.join(dir));
    }
    let mut found = false;

            for pathc in serc {
                if (!pathc.exists()) {
                    continue;
                }
                for entypress in WalkDir::new(pathc).min_depth(1).max_depth(1) {
                    if let Ok(entry) = entypress {
                        if entry.file_type().is_dir() {
                            match project {
                                Some(name) if entry.file_name().to_string_lossy() == name => {
                                    println!("{}", entry.path().display());
                                    found = true;
                                }
                                None => {
                                    println!("{}", entry.path().display());
                                    found = true;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                if !found {
                    let marker = finder(lang);
                    for entry in WalkDir::new(&paths).min_depth(1).max_depth(4) {
                       if let Ok(entry) = entry {
                           if entry.file_name().to_string_lossy() == marker {
                               if let Some(parent) = entry.path().parent() {
                                   match project {
                                       Some(name) if parent.file_name().map_or(false, |n| n.to_string_lossy() == name) => {
                                           println!("{}", parent.display());
                                       }
                                       None => println!("{}", parent.display()),
                                       _ => {},
                                   }
                               }
                           }
                       }
                    }
                }
            }

        }







