use std::collections::HashSet;
use sysinfo::{ProcessExt, System, SystemExt};
use zip::write::FileOptions;
use zip::result::{ZipError,ZipResult};
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use argparse::{ArgumentParser, Store};

fn main() {
    let mut zipfile = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("compresses all process binaries into a zip file");
        ap.refer(&mut zipfile).add_argument("zipfile", Store, "name of the destination zip file").required();
        ap.parse_args_or_exit();
    }

    
    let zipfile = PathBuf::from(zipfile);
    let binaries = get_process_binaries();
    match write_zip(zipfile, &binaries) {
        Err(why)    => println!("failed: {}", why),
        Ok(_)       => ()
    }
}

fn get_process_binaries() -> HashSet<PathBuf> {
    let sys = System::new_all();
    let mut binaries:HashSet<PathBuf> = HashSet::new();
    for (_pid, process) in sys.get_processes() {
        let path = process.exe();
        if ! binaries.contains(path) {
            binaries.insert(path.to_path_buf());
        }
    }
    binaries
}

fn write_zip(zipfile: PathBuf, binaries: &HashSet<PathBuf>) -> ZipResult<()> {
    let path = std::path::Path::new(&zipfile);
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    for p in binaries {
        let pstr = match p.to_str() {
            Some(v) => v,
            None    => return ZipResult::Err(ZipError::FileNotFound),
        };
        zip.start_file(pstr, options)?;
        let mut f = File::open(p)?;
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer)?;
        zip.write_all(&*buffer)?;
    }
    zip.finish()?;
    Result::Ok(())
}
