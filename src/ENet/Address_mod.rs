#[cfg(feature = "ENet+Address")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Address {
    pub nativeAddress: crate::ENet::ENetAddress,
}
#[cfg(feature = "ENet+Address")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::Address => "ENet"."Address"
);
#[cfg(feature = "ENet+Address")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::Address {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+Address")]
impl crate::ENet::Address {
    pub fn GetHost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHost",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetIP(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetIP",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetHost(
        &mut self,
        hostName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetHost",
            (hostName),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetIP(
        &mut self,
        ip: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetIP",
            (ip),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        address: crate::ENet::ENetAddress,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (address),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_NativeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::ENetAddress> {
        let __cordl_ret: crate::ENet::ENetAddress = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NativeData",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Port(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Port",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_NativeData(
        &mut self,
        value: crate::ENet::ENetAddress,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_NativeData",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Port(
        &mut self,
        value: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Port",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}