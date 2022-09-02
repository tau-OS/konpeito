project "konpeito" {
    rpmbuild {
        mode = "cargo"
        //package = "."
        build_deps = ["openssl-devel", "rust-packaging"]
    }
}