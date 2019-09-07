fn main() {
    let root = "vendor/tolk";
    cc::Build::new()
        .cpp(true)
        .define("_EXPORTING", "")
        .define("UNICODE", "")
        .file(format!("{}/src/Tolk.cpp", root))
        .file(format!("{}/src/ScreenReaderDriverJAWS.cpp", root))
        .file(format!("{}/src/ScreenReaderDriverNVDA.cpp", root))
        .file(format!("{}/src/ScreenReaderDriverSA.cpp", root))
        .file(format!("{}/src/ScreenReaderDriverSNova.cpp", root))
        .file(format!("{}/src/ScreenReaderDriverWE.cpp", root))
        .file(format!("{}/src/ScreenReaderDriverZT.cpp", root))
        .file(format!("{}/src/ScreenReaderDriverSAPI.cpp", root))
        .file(format!("{}/src/fsapi.c", root))
        .file(format!("{}/src/wineyes.c", root))
        .file(format!("{}/src/zt.c", root))
        .compile("tolk");
    println!("cargo:rustc-flags=-l User32 -l Ole32 -l OleAut32");
}
