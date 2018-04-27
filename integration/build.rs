use std::process::Command;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

use std::fs::read_dir;

fn main() {
    let install = Command::new("bash")
        .arg("install-cc65.sh")
        .status()
        .expect("Failed to install cc65 dependency");
    if !install.success() {
        panic!(
            format!(
                "Failed to install cc65 dependency\nstatus: {}",
                install
            ));
    }
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("test_definitions.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(b"use integration::rom_test;\n").unwrap();

    compile_dir(&mut f, "ca65");
    compile_dir(&mut f, "ca65/vbl_timing");
    compile_dir(&mut f, "ca65/cpu_dummy_reads");
    compile_dir(&mut f, "ca65/instr_tests");
}

fn compile_dir(out_file: &mut File, directory: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let files = read_dir(directory).unwrap()
        .map(|f| f.unwrap().path())
        .filter(|f| f.file_name().map(|name| name.to_string_lossy().ends_with(".s")).unwrap_or(false));


    for file in files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        let name = &file_name[0..(file_name.len()-2)];
        let assemble = Command::new(".cc65/bin/ca65")
                            .arg("-o")
                            .arg(format!("{}/{}.o", out_dir, name))
                            .arg("-I")
                            .arg(format!("{}/common", directory))
                            .arg(&file)
                            .output()
                            .expect(format!("Failed to assemble {}", file_name).as_str());
        if !assemble.status.success() {
            panic!(
                format!(
                    "Failed to assemble {}\nstatu: {}\nstdout: {}\nstderr: {}", 
                    file_name, 
                    assemble.status, 
                    String::from_utf8_lossy(&assemble.stdout), 
                    String::from_utf8_lossy(&assemble.stderr)
                ));
        }
        let linking = Command::new(".cc65/bin/ld65")
                            .args(&["-C", &(directory.to_owned() + "/nes.cfg"), "-o"])
                            .arg(format!("{}/{}.nes", out_dir, name))
                            .arg(format!("{}/{}.o", out_dir, name))
                            .output()
                            .expect(format!("Failed to assemble {}", name).as_str());
        if !linking.status.success() {
            panic!(
                format!(
                    "Failed to link file {}\nStdout: {}\nstderr: {}",
                    file_name,
                    String::from_utf8_lossy(&linking.stdout),
                    String::from_utf8_lossy(&linking.stderr)
                ));
        }
        out_file.write_all(format!(r#"
            #[test]
            pub fn {}() {{
                rom_test::test("{}/{}.nes");
            }}
        "#, name, out_dir, name).as_bytes()).unwrap();
    }
}
