corrosion_import_crate(
    MANIFEST_PATH
    Cargo.toml
    NO_STD
)

add_dependencies(
    cargo-prebuild_hello_rs
    BINDINGS_RS
)

corrosion_experimental_cbindgen(
    TARGET
    hello_rs
    HEADER_NAME
    hello_rs.h
    FLAGS
    --lang=c
)