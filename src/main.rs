// TODO: Try to get this to work on Windows. In theory it shouldn't require too
// many changes except for the shell part, which I guess has to use cmd.exe or
// whatever. Or maybe we could just require a Bourne-compatible-ish shell on
// all platforms.
#[cfg(target_os = "linux")]
pub mod data;

fn main() {
    unimplemented!();
}
