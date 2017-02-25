#[link(name = "tolk")]
extern "C" {
    pub fn Tolk_Load();
    pub fn Tolk_IsLoaded() -> bool;
    pub fn Tolk_Unload();
}

#[test]
fn load_unload() {
    unsafe {
        Tolk_Load();
        assert_eq!(Tolk_IsLoaded(), true);
        Tolk_Unload();
        assert_eq!(Tolk_IsLoaded(), false);
    }
}
