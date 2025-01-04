#[cfg(feature = "System+Globalization+GregorianCalendarTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GregorianCalendarTypes {
    #[default]
    Arabic = 10i32,
    Localized = 1i32,
    MiddleEastFrench = 9i32,
    TransliteratedEnglish = 11i32,
    TransliteratedFrench = 12i32,
    USEnglish = 2i32,
}
#[cfg(feature = "System+Globalization+GregorianCalendarTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::GregorianCalendarTypes =>
    "System.Globalization"."GregorianCalendarTypes"
);
