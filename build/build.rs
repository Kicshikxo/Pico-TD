extern crate embed_resource;

fn main() {
    #[cfg(target_os = "windows")]
    embed_resource::compile("build/windows/icon.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
