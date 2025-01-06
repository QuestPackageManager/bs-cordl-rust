#[cfg(feature = "System+Net+Sockets+IPPacketInformation")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IPPacketInformation {
    pub address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    pub networkInterface: i32,
}
#[cfg(feature = "System+Net+Sockets+IPPacketInformation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::IPPacketInformation =>
    "System.Net.Sockets"."IPPacketInformation"
);
#[cfg(feature = "System+Net+Sockets+IPPacketInformation")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::Sockets::IPPacketInformation {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+Sockets+IPPacketInformation")]
impl crate::System::Net::Sockets::IPPacketInformation {
    pub fn Equals(
        &mut self,
        comparand: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (comparand),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
