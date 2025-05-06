use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

fn run_template(path: PathBuf, source: &str, buffer: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;

    println!("cargo:rerun-if-changed={source}");

    for l in std::fs::read_to_string(source)?.lines() {
        let mut l = l
            .replace("crate::dynamic::LzssDyn", "crate::generic::Lzss")
            .replace(
                "impl LzssDyn {",
                "impl<const EI: usize, const EJ: usize, const C: u8, const N: usize, const N2: usize> Lzss<EI, EJ, C, N, N2> {",
            )
            .replace("&self,", "")
            .replace("buffer: &mut [u8],", buffer)
            .replace("self.ei", "EI")
            .replace("self.ej", "EJ")
            .replace("self.f()", "Self::F")
            .replace("self.n()", "N")
            .replace("self.p", "Self::P")
            ;
        l.push('\n');
        file.write_all(l.as_bytes())?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let out_dir =
        env::var_os("OUT_DIR").ok_or_else(|| Error::new(ErrorKind::Other, "no OUT_DIR"))?;
    let out_dir = Path::new(&out_dir);

    run_template(
        out_dir.join("generic-compress.rs"),
        "src/dynamic/compress.rs",
        "buffer: &mut [u8; N2],",
    )?;

    run_template(
        out_dir.join("generic-decompress.rs"),
        "src/dynamic/decompress.rs",
        "buffer: &mut [u8; N],",
    )?;

    Ok(())
}
