#[cfg(feature = "Newtonsoft+Json+Required")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Required {
    AllowNull = 1i32,
    Always = 2i32,
    Default = 0i32,
    DisallowNull = 3i32,
}
#[cfg(feature = "Newtonsoft+Json+Required")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Required => "Newtonsoft.Json"
    ."Required"
);
