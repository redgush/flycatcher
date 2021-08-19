use std::process::Command;

/// Options to configure the parser with.
pub struct LinkerOptions {

    /// The path of the executable that the linker will generate.
    pub output_path: Option<String>,

}

/// Links a list of file paths with the chosen linker, which defaults to the GCC linker.
/// Returns whether or not the linking process was successful.
pub fn link(files: Vec<String>, options: LinkerOptions) -> bool {
    let mut args = files;

    if let Some(path) = options.output_path {
        args.push("-o".into());
        args.push(path);
    }

    let res = Command::new("gcc")
        .args(&args[..])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    
    res.success()
}