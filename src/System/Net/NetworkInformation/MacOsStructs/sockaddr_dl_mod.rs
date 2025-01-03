#[cfg(feature = "System+Net+NetworkInformation+MacOsStructs+sockaddr_dl")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct sockaddr_dl {
    pub sdl_len: u8,
    pub sdl_family: u8,
    pub sdl_index: u16,
    pub sdl_type: u8,
    pub sdl_nlen: u8,
    pub sdl_alen: u8,
    pub sdl_slen: u8,
    pub sdl_data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsStructs+sockaddr_dl")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::MacOsStructs::sockaddr_dl =>
    "System.Net.NetworkInformation.MacOsStructs"."sockaddr_dl"
);
#[cfg(feature = "System+Net+NetworkInformation+MacOsStructs+sockaddr_dl")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::MacOsStructs::sockaddr_dl {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsStructs+sockaddr_dl")]
impl crate::System::Net::NetworkInformation::MacOsStructs::sockaddr_dl {
    pub fn Read(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Read",
            (ptr),
        )?;
        Ok(__cordl_ret.into())
    }
}
