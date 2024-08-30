use colored::*;
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use clap::Parser;
use toml_edit::DocumentMut;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the pyproject.toml file
    pyproject: PathBuf,

    /// Whether to overwrite the existing pyproject.toml - defaults to false
    #[arg(long, action, default_value_t = false)]
    no_overwrite: bool,

    /// Don't create a backup file of the original rye config - defaults to false
    #[arg(long, action, default_value_t = false)]
    no_backup: bool,

    /// Just print the output file
    #[arg(short, long, default_value_t = false)]
    print: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if !args.no_backup && !args.no_overwrite && !args.print {
        // create a back up copy
        let mut bc_buf = String::new();
        let mut bc_file = fs::File::open(args.pyproject.as_path())?;
        bc_file.read_to_string(&mut bc_buf)?;
        fs::File::create("pyproject-rye.toml")?.write_all(bc_buf.as_bytes())?;
    }

    let mut buf = String::new();
    let mut input_file = fs::File::open(args.pyproject.as_path())?;
    input_file.read_to_string(&mut buf)?;

    let out_file = {
        if !args.print {
            if !args.no_overwrite {
                fs::File::options()
                    .write(true)
                    .truncate(true)
                    .open(args.pyproject.as_path())
                    .ok()
            } else {
                fs::File::options()
                    .create_new(true)
                    .write(true)
                    .open("pyproject-uv.toml")
                    .ok()
            }
        } else {
            None
        }
    };

    let mut document: DocumentMut = buf.parse()?;
    let uv_version = rye_uv::get_tool_version("uv")?;
    rye_uv::convert(&mut document, uv_version)?;
    let document_string = document.to_string();
    if document_string.trim().is_empty() {
        eprintln!("The file seems to be empty");
    } else {
        if args.print {
            println!("{}", document_string)
        }
        if let Some(mut f) = out_file {
            let _ = f
                .write(document_string.as_bytes())
                .expect("Cannot write to file");
            println!("{}", "All set ✨🌿👋".bold().green());
        }
    }
    Ok(())
}
