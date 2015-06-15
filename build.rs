extern crate tango;

fn main() {
    tango::process_root().unwrap();

    // We must run pandoc's processing after tango since tango may
    // generate `.md` files that we want to process.
    pandoc::process_root().unwrap();
}

mod pandoc {
    use std::io;
    use std::process::Command;

    pub fn process_root() -> io::Result<()> {
        let mut mk_slide_dir = Command::new("mkdir");
        mk_slide_dir.args(&["-p", "target/slides/"]);
        mk_slide_dir.status().unwrap();

        for name in &["path_to_gecko"] {
            let src_path = &format!("src/tutorial/{}.md", name);
            let tgt_path = &format!("target/slides/{}.html", name);
            let mut pandoc = Command::new("pandoc");
            pandoc
                .args(&["-t", "revealjs"])
                .args(&["-s", src_path])
                .args(&["-o", tgt_path]);
            pandoc.status().unwrap();
        }

        Ok(())
    }
}

