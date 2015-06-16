extern crate tango;

fn main() {
    tango::process_root().unwrap();

    // We must run pandoc's processing after tango since tango may
    // generate `.md` files that we want to process.
    pandoc::process_root().unwrap();
}

mod pandoc {
    use std::env;
    use std::fs;
    use std::io;
    use std::process::Command;

    pub fn process_root() -> io::Result<()> {
        let mut mk_slide_dir = Command::new("mkdir");
        mk_slide_dir.args(&["-p", "target/slides/"]);
        mk_slide_dir.status().ok()
            .expect("we should be able to ensure `target/slides/` exists");

        for name in &["path_to_gecko"] {
            let src_dir_path = &format!("src/tutorial/{}", name);
            let mut src_paths = Vec::new();
            for entry in try!(fs::read_dir(src_dir_path)) {
                let entry = try!(entry);
                if let Some("md") = entry.path().extension().and_then(|p|p.to_str()) {
                    src_paths.push(entry.path());
                }
            }
            let tgt_path = &format!("target/slides/{}.html", name);
            let mut pandoc = Command::new("pandoc");
            pandoc
                .args(&["-t", "revealjs"])
                .args(&["-V", "theme=black"])
                // .args(&["--highlight-style=espresso"])
                // .args(&["--highlight-style=pygments"])
                .args(&["--highlight-style=kate"])
                .args(&["--css", "../../slide-style.css"])
                .args(&["-o", tgt_path])
                .args(&["-s"]);
            for p in src_paths {
                pandoc.arg(p);
            }
            let command = format!("{:?}", pandoc);
            match pandoc.output() {
                Ok(ref output) if output.status.success() => {}
                Ok(ref output) => {
                    panic!("something went wrong running pandoc; \
                            command: {} current_dir: {} exit status: {:?} stdout: {} stderr: {}",
                           command,
                           env::current_dir().unwrap().display(),
                           output.status.code(),
                           String::from_utf8_lossy(&output.stdout),
                           String::from_utf8_lossy(&output.stderr),
                           );
                }
                Err(e) => {
                    panic!("something went wrong running pandoc; \
                            command: {} current_dir: {} err: {} PATH: {:?}",
                           command,
                           env::current_dir().unwrap().display(),
                           e,
                           env::var_os("PATH"));
                }
            }
        }

        Ok(())
    }
}

