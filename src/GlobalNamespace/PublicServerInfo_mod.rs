#[cfg(feature = "PublicServerInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PublicServerInfo {
    pub code: *mut quest_hook::libil2cpp::Il2CppString,
    pub currentPlayerCount: i32,
}
#[cfg(feature = "PublicServerInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PublicServerInfo => ""
    ."PublicServerInfo"
);
#[cfg(feature = "PublicServerInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PublicServerInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PublicServerInfo")]
impl crate::GlobalNamespace::PublicServerInfo {
    pub fn Deserialize(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PublicServerInfo> {
        let __cordl_ret: crate::GlobalNamespace::PublicServerInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        currentPlayerCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (code, currentPlayerCount),
        )?;
        Ok(__cordl_ret.into())
    }
}
