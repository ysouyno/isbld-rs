use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

extern crate serde;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Data {
    is_home: String,
    project: String,
    winrar: String,
    out: String,
}

#[derive(Debug)]
struct Param {
    compiler: String,
    rulfiles: String,
    libraries: String,
    linkpaths: String,
    includeifx: String,
    includeisrt: String,
    includescript: String,
    definitions: String,
    switches: String,
    builder: String,
    installproject: String,
    disk1: String,
}

fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

fn gen_config(config: &str) -> std::result::Result<std::fs::File, std::io::Error> {
    println!("gen_config called");
    let data = Data {
        // Where to find InstallShield, it is a directory location, like:
        // "C:\\Program Files (x86)\\InstallShield\\2018"
        is_home: "C:\\Program Files (x86)\\InstallShield\\2018".to_owned(),
        project: "Your Project Name.ism".to_owned(),
        // Where to find WinRAR.exe
        winrar: "C:\\Program Files (x86)\\WinRAR\\WinRAR.exe".to_owned(),
        out: "out.exe".to_owned(),
    };
    let data = serde_json::to_string_pretty(&data);
    let mut file = std::fs::File::create(config).expect("create failed");
    file.write_all(data.unwrap().as_bytes())
        .expect("write failed");
    Ok(file)
}

fn get_param() -> std::io::Result<Param> {
    let cur_dir = env::current_exe()?;
    let path = std::path::Path::new(&cur_dir);
    let parent = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let config = format!("{}\\{}.json", parent.display(), file_stem);
    println!("{}", config);

    let mut file = std::fs::File::open(&config);
    match file {
        Err(e) => {
            println!("{}", e);
            gen_config(&config).unwrap();
        }
        _ => (),
    }

    file = std::fs::File::open(&config);
    let mut json_str = String::new();
    file.unwrap().read_to_string(&mut json_str).unwrap();
    let json: Data = serde_json::from_str(&json_str).unwrap();
    if !path_exists(json.is_home.as_str()) {
        panic!(
            "{} not exists, please edit {}.json",
            json.is_home, file_stem
        );
    }

    if !path_exists(json.winrar.as_str()) {
        panic!("{} not exists, please edit {}.json", json.winrar, file_stem);
    }

    let param = Param {
        compiler: format!("{}\\System\\Compile.exe", json.is_home),
        rulfiles: format!("{}\\Script Files\\Setup.rul", parent.display()),
        libraries: "\"isrt.obl\" \"ifx.obl\"".to_owned(),
        linkpaths: format!(
            "-LibPath\"{}\\Script\\Ifx\\Lib\" -LibPath\"{}\\Script\\Isrt\\Lib\"",
            json.is_home, json.is_home
        ),
        includeifx: format!("{}\\Script\\Ifx\\Include", json.is_home),
        includeisrt: format!("{}\\Script\\Isrt\\Include", json.is_home),
        includescript: format!("{}\\Script Files", parent.display()),
        definitions: "".to_owned(),
        switches: "-w50 -e50 -v3 -g".to_owned(),
        builder: format!("{}\\System\\ISCmdBld.exe", json.is_home),
        installproject: format!("{}\\{}", parent.display(), json.project),
        disk1: format!(
            "{}\\Media\\EIOSetup_SCH\\Disk Image\\Disk1",
            parent.display()
        ),
    };
    println!("{:?}", param);

    Ok(param)
}

fn run_cmd(command: &str) -> Result<(), Error> {
    println!("run: {}", command);
    let command_vec: Vec<String> = command.split_whitespace().map(str::to_string).collect();
    let stdout = Command::new("CMD")
        // first of all you should exec `chcp 65001`
        .arg("/C")
        .args(command_vec)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    Ok(())
}

fn build(param: Param) -> Result<(), Error> {
    // let chcp_cmd = "chcp 65001".to_owned();
    // run_cmd(chcp_cmd.as_str()).unwrap();
    let compile_cmd = format!(
        "\"\"{}\" \"{}\" {} {} -I\"{}\" -I\"{}\" -I\"{}\" {} {}\"",
        param.compiler.as_str(),
        param.rulfiles.as_str(),
        param.libraries.as_str(),
        param.linkpaths.as_str(),
        param.includeifx.as_str(),
        param.includeisrt.as_str(),
        param.includescript.as_str(),
        param.definitions.as_str(),
        param.switches.as_str()
    );
    run_cmd(compile_cmd.as_str()).unwrap();
    let build_cmd = format!(
        "\"\"{}\" -p \"{}\"\"",
        param.builder.as_str(),
        param.installproject.as_str()
    );
    run_cmd(build_cmd.as_str()).unwrap();

    Ok(())
}

fn main() {
    let param = get_param().unwrap();
    build(param).unwrap();
}
