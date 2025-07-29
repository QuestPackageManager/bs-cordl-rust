#[cfg(feature = "cordl_class_LiteNetLib+PacketProperty")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PacketProperty {
    #[default]
    Ack = 2u8,
    Broadcast = 11u8,
    Channeled = 1u8,
    ConnectAccept = 6u8,
    ConnectRequest = 5u8,
    Disconnect = 7u8,
    Empty = 17u8,
    InvalidProtocol = 15u8,
    Merged = 12u8,
    MtuCheck = 9u8,
    MtuOk = 10u8,
    NatMessage = 16u8,
    PeerNotFound = 14u8,
    Ping = 3u8,
    Pong = 4u8,
    ShutdownOk = 13u8,
    UnconnectedMessage = 8u8,
    Unreliable = 0u8,
}
#[cfg(feature = "cordl_class_LiteNetLib+PacketProperty")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::PacketProperty {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "PacketProperty";
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
#[cfg(feature = "cordl_class_LiteNetLib+PacketProperty")]
unsafe impl quest_hook::libil2cpp::Argument for crate::LiteNetLib::PacketProperty {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_LiteNetLib+PacketProperty")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::LiteNetLib::PacketProperty {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_LiteNetLib+PacketProperty")]
unsafe impl quest_hook::libil2cpp::Returned for crate::LiteNetLib::PacketProperty {
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
#[cfg(feature = "cordl_class_LiteNetLib+PacketProperty")]
unsafe impl quest_hook::libil2cpp::Return for crate::LiteNetLib::PacketProperty {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
