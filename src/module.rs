use std::{
    ffi::OsStr,
    fs::{read_dir, read_to_string},
    io::Result as IoResult,
    path::{Path, PathBuf},
};

#[derive(Default, Clone, Debug)]
pub struct Module {
    pub name: String,
    pub body: Option<String>,
    pub sub_modules: Vec<Module>,
}

impl Module {
    pub fn new(name: &OsStr) -> Module {
        Module {
            name: name.to_os_string().into_string().unwrap(),
            body: None,
            sub_modules: Vec::new(),
        }
    }
}

/// Read module from directory recursively.
/// `path` has to be directory.
pub fn read_module(path: impl AsRef<Path>) -> IoResult<Module> {
    let module = Module::new(path.as_ref().file_name().unwrap());
    read_module_dir(path, module)
}

/// Read module from directory recursively.
/// `path` has to be directory.
/// `module` might be initialized.
///
/// Let say, `path` is `/opt/src/var` and `/opt/src/var.rs` exists, `module` is initialized with
/// contents of `/opt/src/var.rs`.
/// And then, this function reads contents in `/opt/src/var` directory.
fn read_module_dir(path: impl AsRef<Path>, mut module: Module) -> IoResult<Module> {
    // try read `path/mod.rs`
    if module.body.is_none() {
        let mod_file_path = {
            let mut path_buf = path.as_ref().to_path_buf();
            path_buf.push("mod.rs");
            path_buf
        };
        module.body = read_to_string(mod_file_path).ok();
    }

    // read sub_modules
    let (files, dirs): (Vec<PathBuf>, Vec<PathBuf>) = read_dir(path)?
        .collect::<IoResult<Vec<_>>>()?
        .into_iter()
        .map(|entry| entry.path())
        // ignore unrelated files
        .filter(|path| {
            (path.extension() == Some(OsStr::new("rs"))
                && path.file_name() != Some(OsStr::new("mod.rs")))
                || path.is_dir()
        })
        .partition(|p| p.extension() == Some(OsStr::new("rs")));

    // read files
    let mut file_mods = files
        .iter()
        .map(|path| {
            Ok(Module {
                name: path
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
                body: Some(read_to_string(path)?),
                sub_modules: Vec::new(),
            })
        })
        .collect::<IoResult<Vec<Module>>>()?;

    let mut dir_mods = dirs
        .iter()
        .map(|path| {
            let module = match file_mods
                .iter()
                .position(|f| f.name.as_str() == path.file_name().unwrap())
            {
                Some(pos) => file_mods.remove(pos),
                None => Module::new(path.file_name().unwrap()),
            };

            read_module_dir(path, module)
        })
        .collect::<IoResult<Vec<Module>>>()?;

    file_mods.append(&mut dir_mods);
    module.sub_modules = file_mods;

    Ok(module)
}
