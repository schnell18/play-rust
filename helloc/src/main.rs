// Note the lack of the `#[link]` attribute.
// We're delegating the resposibility of selecting
// what to link to over to the build script rather
// than hardcoding it in the source file.
extern {
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
}
