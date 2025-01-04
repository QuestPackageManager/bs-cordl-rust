#[cfg(feature = "Newtonsoft+Json+Linq+LineInfoHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineInfoHandling {
    #[default]
    Ignore = 0i32,
    Load = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+Linq+LineInfoHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::LineInfoHandling =>
    "Newtonsoft.Json.Linq"."LineInfoHandling"
);
