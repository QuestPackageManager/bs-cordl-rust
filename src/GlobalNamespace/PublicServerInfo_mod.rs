#[cfg(feature = "PublicServerInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PublicServerInfo {
    pub code: *mut crate::System::String,
    pub currentPlayerCount: i32,
}
#[cfg(feature = "PublicServerInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for PublicServerInfo => ""."PublicServerInfo"
);
#[cfg(feature = "PublicServerInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for PublicServerInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PublicServerInfo")]
impl PublicServerInfo {
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        code: *mut crate::System::String,
        currentPlayerCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (code, currentPlayerCount),
        )?;
        Ok(__cordl_ret)
    }
}
