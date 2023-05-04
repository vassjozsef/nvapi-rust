fn main() {
    let path = "C:\\Users\\jozsef\\Desktop\\R530-developer-2";
    println!(r"cargo:rustc-link-search={}\amd64\", path);
}
