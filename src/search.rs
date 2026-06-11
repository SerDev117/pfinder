use std::env::set_current_dir;
use std::path::PathBuf;
use crate::Language;
use walkdir::{WalkDir, DirEntry};
use dirs;


pub fn find(lang: Language, maybe: Option<&str>) {

        finder(lang, maybe);
    }


fn finder(lang: Language, project: Option<&str>) {
    let paths = dirs::home_dir().unwrap();
    let idef = match lang {
        Language::cpp => Some("ClionProjects"),
        Language::java => Some("IdeaProjects"),
        Language::js => Some("WebstormProjects"),
        Language::python => Some("PycharmProjects"),
        Language::rust => Some("RustroverProjects"),
        _ => None,
    };




    let mut serc: Vec<PathBuf> = Vec::new();
    serc.push(paths.join("dev"));
    serc.push(paths.join("AndroidStudioProjects"));
    serc.push(paths.join("projects"));
    serc.push(paths.join("myproject"));
serc.push(paths.join("code"));
    if let Some(dir) = idef {
        if (std::fs::read_dir(dir).map_or(true, |mut d| d.next().is_none())) {
            serc.push(paths.join(PathBuf::from(dir)));
            for pathc in serc {
                if (!pathc.exists()) {
                    continue;
                }
                for entypress in WalkDir::new(&pathc).min_depth(1).max_depth(1) {
                    if let Ok(entry) = entypress {
                        if entry.file_type().is_dir() {
                            match project {
                                Some(name) if entry.file_name().to_string_lossy() == name => {
                                    println!("{}", entry.path().display());
                                }
                                None => {
                                    println!("{}", entry.path().display());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }



}