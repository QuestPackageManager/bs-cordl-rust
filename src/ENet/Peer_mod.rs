#[cfg(feature = "ENet+Peer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Peer {
    pub nativePeer: crate::System::IntPtr,
    pub nativeID: u32,
}
#[cfg(feature = "ENet+Peer")]
unsafe impl quest_hook::libil2cpp::Type for crate::ENet::Peer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "ENet";
    const CLASS_NAME: &'static str = "Peer";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "ENet+Peer")]
unsafe impl quest_hook::libil2cpp::Argument for crate::ENet::Peer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ENet+Peer")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::ENet::Peer {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "ENet+Peer")]
unsafe impl quest_hook::libil2cpp::Returned for crate::ENet::Peer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "ENet+Peer")]
unsafe impl quest_hook::libil2cpp::Return for crate::ENet::Peer {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "ENet+Peer")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::Peer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+Peer")]
impl crate::ENet::Peer {
    pub fn ConfigureThrottle(
        &mut self,
        interval: u32,
        acceleration: u32,
        deceleration: u32,
        threshold: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConfigureThrottle",
            (interval, acceleration, deceleration, threshold),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect(
        &mut self,
        data: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Disconnect",
            (data),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectLater(
        &mut self,
        data: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DisconnectLater",
            (data),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectNow(
        &mut self,
        data: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DisconnectNow",
            (data),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Ping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Ping",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PingInterval(
        &mut self,
        interval: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PingInterval",
            (interval),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive(
        &mut self,
        channelID: quest_hook::libil2cpp::ByRefMut<u8>,
        packet: quest_hook::libil2cpp::ByRefMut<crate::ENet::Packet>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Receive",
            (channelID, packet),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Send(
        &mut self,
        channelID: u8,
        packet: quest_hook::libil2cpp::ByRefMut<crate::ENet::Packet>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Send",
            (channelID, packet),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfNotCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ThrowIfNotCreated",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Timeout(
        &mut self,
        timeoutLimit: u32,
        timeoutMinimum: u32,
        timeoutMaximum: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Timeout",
            (timeoutLimit, timeoutMinimum, timeoutMaximum),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (peer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BytesReceived(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_BytesReceived",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BytesSent(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_BytesSent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Data(&mut self) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Data",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ID(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IP(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_IP", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSet(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsSet",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastReceiveTime(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LastReceiveTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastRoundTripTime(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LastRoundTripTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastSendTime(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LastSendTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MTU(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MTU",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NativeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NativeData",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PacketsLost(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PacketsLost",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PacketsSent(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PacketsSent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PacketsThrottle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PacketsThrottle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Port(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Port",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RoundTripTime(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RoundTripTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_State(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::PeerState> {
        let __cordl_ret: crate::ENet::PeerState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_State",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Data(
        &mut self,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Data",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NativeData(
        &mut self,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_NativeData",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
