use std::{fs, path::Path};

const COPY_DIR: &'static str = "../assets";

fn main() {
    use std::env;
    use std::path::PathBuf;
    println!("cargo:rerun-if-changed=src/ui.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/ui.fl", out_path.join("ui.rs").to_str().unwrap())
        .expect("Failed to generate rust from fl file!");

    // Request the output directory
    let out = PathBuf::from(format!("{}/{}", &out_path.parent().unwrap().parent().unwrap().to_str().unwrap(), COPY_DIR));
    


    // If it is already in the output directory, delete it and start over
    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }

    // Create the out directory
    fs::create_dir(&out).unwrap();

    // Copy the directory
    copy_dir(COPY_DIR, &out);
}

fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        } else { /* Skip other content */
        }
    }
}