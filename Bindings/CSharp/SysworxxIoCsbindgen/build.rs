fn main() {
    csbindgen::Builder::default()
        .input_bindgen_file("../../Rust/sysworxx_io.rs")
        .csharp_namespace("Sysworxx")
        .csharp_class_name("SysworxxIoSys")
        .csharp_dll_name("sysworxx_io.so")
        .generate_csharp_file("../SysworxxIo/SysworxxIoNative.cs")
        .expect("generated C# bindinngs");
}
