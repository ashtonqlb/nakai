fn main() {
    if cfg!(target_os = "windows") {
        panic!("WARNING! While this program *may* work correctly on Windows, we offer NO SUPPORT WHATSOEVER.\nThis program does, however, run perfectly fine in Docker / on WSL.\nPlease do not open any GitHub issues if you disable this compile guard. You are ON YOUR OWN");
    }
}