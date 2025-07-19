fn main() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=Simple_Caro");
    println!("cargo:rustc-link-lib=stdc++");
}