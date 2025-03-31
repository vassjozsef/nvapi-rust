fn main() {
    let path = "D:\\R560-developer";
    println!(r"cargo:rustc-link-search={}\amd64\", path);
}
