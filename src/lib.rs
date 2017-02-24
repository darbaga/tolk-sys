#[link(name = "tolk")]
extern {
    fn Tolk_Load();
    fn Tolk_IsLoaded() -> bool;
    fn Tolk_Unload();
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
