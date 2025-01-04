#[cfg(feature = "System+Globalization+CultureTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CultureTypes {
    #[default]
    AllCultures = 7i32,
    FrameworkCultures = 64i32,
    InstalledWin32Cultures = 4i32,
    NeutralCultures = 1i32,
    ReplacementCultures = 16i32,
    SpecificCultures = 2i32,
    UserCustomCulture = 8i32,
    WindowsOnlyCultures = 32i32,
}
#[cfg(feature = "System+Globalization+CultureTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::CultureTypes =>
    "System.Globalization"."CultureTypes"
);
