use inwelling::*;

use std::{env, path::PathBuf};

fn main() {
    let mut output = inwelling(Opts {
        watch_manifest: true,
        watch_rs_files: true,
        dump_rs_paths: true,
    })
    .sections
    .into_iter()
    .fold(
        String::from("lens_rs_derive::scan_optics_from_source_files!{\n"),
        |mut output, section| {
            section.rs_paths.unwrap().iter().for_each(|rs_path| {
                let rs_path = rs_path.to_str().unwrap();

                if cfg!(windows) {
                    output.push_str(&format!("    \"{}\",\n", rs_path.replace("\\", "\\\\")));
                } else {
                    output.push_str(&format!("    \"{}\",\n", rs_path));
                }
            });
            output
        },
    );
    output.push('}');

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("$OUT_DIR should exist."));
    std::fs::write(out_path.join("optics.rs"), output).expect("optics.rs should be generated.");
}
