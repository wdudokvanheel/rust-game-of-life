extern crate embed_resource;

fn main() {
    embed_resource::compile("src/assets/resources.rc", embed_resource::NONE);
}
