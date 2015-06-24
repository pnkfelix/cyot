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

    enum PandocTarget {
        Slides,
        Exercises,
    }

    impl PandocTarget {
        fn subdir(&self) -> &'static str {
            match *self {
                PandocTarget::Slides => "slides",
                PandocTarget::Exercises => "exercises",
            }
        }

        fn add_args(&self, pandoc: &mut Command, tgt_path: &str) {
            match *self {
                PandocTarget::Slides =>
                    add_slide_args(pandoc),
                PandocTarget::Exercises =>
                    add_exercise_args(pandoc),
            };
            pandoc
                .args(&["-o", tgt_path])
                .args(&["-s"]);
        }
    }

    fn add_slide_args(pandoc: &mut Command) {
        {
            pandoc
                .args(&["-t", "revealjs"])
                .args(&["-V", "theme=simple"])
                // .args(&["--highlight-style=espresso"])
                // .args(&["--highlight-style=pygments"])
                .args(&["--highlight-style=kate"])
                .args(&["--css", "../../slide-style.css"])
                .args(&["--css", "../../code-style.css"]);
        }
    }

    fn add_exercise_args(pandoc: &mut Command) {
        {
            pandoc
                .args(&["--css", "../../code-style.css"]);
        }
    }

    pub fn process_root() -> io::Result<()> {
        let mut mk_slide_dir = Command::new("mkdir");
        mk_slide_dir.args(&["-p", "target/slides/"]);
        mk_slide_dir.status().ok()
            .expect("we should be able to ensure `target/slides/` exists");

        let mut mk_exercise_dir = Command::new("mkdir");
        mk_exercise_dir.args(&["-p", "target/exercises/"]);
        mk_exercise_dir.status().ok()
            .expect("we should be able to ensure `target/exercises/` exists");

        let slide_sources = ["whistler_rust_intro"];

        for name in &slide_sources {
            try!(run_pandoc(PandocTarget::Slides, name));
        }

        let exercises_sources = ["ex_part_1", "ex_part_3"];
        for name in &exercises_sources {
            try!(run_pandoc(PandocTarget::Exercises, name));
        }

        Ok(())
    }

    fn is_mod_md(entry: &fs::DirEntry) -> bool {
        {
            if let Some("mod.md") = entry.path().file_name().and_then(|p|p.to_str()) {
                true
            } else {
                false
            }
        }
    }

    fn run_pandoc(target: PandocTarget, name: &str) -> io::Result<()> {
        {
            let src_dir_path = &format!("src/tutorial/{}", name);
            let mut src_paths = Vec::new();
            for entry in try!(fs::read_dir(src_dir_path)) {
                let entry = try!(entry);
                if is_mod_md(&entry) { continue; }
                if let Some("md") = entry.path().extension().and_then(|p|p.to_str()) {
                    src_paths.push(entry.path());
                }
            }
            let tgt_path = &format!("target/{}/{}.html", target.subdir(), name);
            let mut pandoc = Command::new("pandoc");
            target.add_args(&mut pandoc, tgt_path);

            src_paths.sort();
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

