#[cfg(feature = "System+Net+DecompressionMethods")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DecompressionMethods {
    #[default]
    Deflate = 2i32,
    GZip = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Net+DecompressionMethods")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::DecompressionMethods =>
    "System.Net"."DecompressionMethods"
);
