extern crate embed_resource;

fn main() {
    embed_resource::compile("app-icon.rc", embed_resource::NONE);
}