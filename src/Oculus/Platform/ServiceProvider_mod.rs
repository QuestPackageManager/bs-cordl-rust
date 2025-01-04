#[cfg(feature = "Oculus+Platform+ServiceProvider")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ServiceProvider {
    #[default]
    Dropbox = 1i32,
    Facebook = 2i32,
    Google = 3i32,
    Instagram = 4i32,
    RemoteMedia = 5i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+ServiceProvider")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::ServiceProvider =>
    "Oculus.Platform"."ServiceProvider"
);
