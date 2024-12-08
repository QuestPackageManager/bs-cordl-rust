#[cfg(feature = "ENet+Event")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Event {
    pub nativeEvent: crate::ENet::ENetEvent,
}
#[cfg(feature = "ENet+Event")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::Event => "ENet"."Event"
);
#[cfg(feature = "ENet+Event")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::Event {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+Event")]
impl crate::ENet::Event {
    pub fn _ctor(
        &mut self,
        event: crate::ENet::ENetEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (event),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ChannelID(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ChannelID",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Data(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Data",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_NativeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::ENetEvent> {
        let __cordl_ret: crate::ENet::ENetEvent = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NativeData",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Packet(&mut self) -> quest_hook::libil2cpp::Result<crate::ENet::Packet> {
        let __cordl_ret: crate::ENet::Packet = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Packet",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Peer(&mut self) -> quest_hook::libil2cpp::Result<crate::ENet::Peer> {
        let __cordl_ret: crate::ENet::Peer = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Peer",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<crate::ENet::EventType> {
        let __cordl_ret: crate::ENet::EventType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_NativeData(
        &mut self,
        value: crate::ENet::ENetEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_NativeData",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
