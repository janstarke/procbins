use std::collections::HashSet;
use sysinfo::{ProcessExt, System, SystemExt};
use zip::write::FileOptions;
use zip::result::{ZipError,ZipResult};
use std::fs::File;
use std::path::PathBuf;
use path_slash::PathBufExt;
use std::io::prelude::*;
use argparse::{ArgumentParser, Store};
use regex::Regex;
use std::borrow::Cow;
use log::{info, warn, error};

struct BinaryStatus {
    files: HashSet<PathBuf>
}

fn main() {
    colog::init();
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
        Err(why)    => error!("failed: {}", why),
        Ok(_)       => ()
    }
}

fn get_process_binaries() -> BinaryStatus {
    let sys = System::new_all();
    let mut binaries = BinaryStatus {
        files: HashSet::new()
    };
    for (_pid, process) in sys.get_processes() {
        let path = process.exe();

        if ! path.exists() {
            warn!("process {}({}) refers to invalid program name, omitting...", process.name(), process.pid());
            continue;
        }

        if ! binaries.files.contains(path) {
            binaries.files.insert(path.to_path_buf());
        }
    }
    binaries
}

fn write_zip(zipfile: PathBuf, binaries: &BinaryStatus) -> ZipResult<()> {
    let path = std::path::Path::new(&zipfile);
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let re_drive = Regex::new(r"^(?P<p>[A-Z]):").unwrap();

    for p in &binaries.files {
        let mut f = match File::open(p) {
            Ok(f) => f,
            Err(why) => {
                error!("error while opening '{}': {}", p.to_str().unwrap(), why);
                continue;
            }
        };
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        let pstr = match p.to_slash() {
            Some(v) => v,
            None    => return ZipResult::Err(ZipError::FileNotFound),
        };
        
        #[cfg(windows)]
        let pstr = match re_drive.replace_all(&pstr[..], "$p") {
            Cow::Borrowed(s)    => String::from(s),
            Cow::Owned(s)     => s,
        };

        info!("adding {}", pstr);
        zip.start_file(pstr, options)?;
        zip.write_all(&*buffer)?;
    }
    zip.finish()?;
    Result::Ok(())
}
