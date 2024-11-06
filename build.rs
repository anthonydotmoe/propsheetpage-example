fn main() {
    let mut res = winres::WindowsResource::new();
    let rc_path = "res/resource.rc";
    println!("cargo:rerun-if-changed={}", rc_path);
    res.set_resource_file(&rc_path);
    res.compile().unwrap();
}