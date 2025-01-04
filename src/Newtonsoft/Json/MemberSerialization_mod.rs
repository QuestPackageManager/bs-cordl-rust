#[cfg(feature = "Newtonsoft+Json+MemberSerialization")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MemberSerialization {
    #[default]
    Fields = 2i32,
    OptIn = 1i32,
    OptOut = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+MemberSerialization")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::MemberSerialization =>
    "Newtonsoft.Json"."MemberSerialization"
);
