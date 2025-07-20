#[cfg(feature = "LiteNetLib+NetEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct NetEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Type: crate::LiteNetLib::NetEvent_EType,
    pub Peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    pub RemoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    pub UserData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Latency: i32,
    pub ErrorCode: crate::System::Net::Sockets::SocketError,
    pub DisconnectReason: crate::LiteNetLib::DisconnectReason,
    pub ConnectionRequest: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::ConnectionRequest,
    >,
    pub DeliveryMethod: crate::LiteNetLib::DeliveryMethod,
    pub DataReader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
}
#[cfg(feature = "LiteNetLib+NetEvent")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::NetEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NetEvent";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "LiteNetLib+NetEvent")]
impl std::ops::Deref for crate::LiteNetLib::NetEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetEvent")]
impl std::ops::DerefMut for crate::LiteNetLib::NetEvent {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetEvent")]
impl crate::LiteNetLib::NetEvent {
    #[cfg(feature = "LiteNetLib+NetEvent+EType")]
    pub type EType = crate::LiteNetLib::NetEvent_EType;
    pub fn New(
        manager: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (manager))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (manager))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+NetEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+NetEvent+EType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetEvent_EType {
    #[default]
    Broadcast = 6i32,
    Connect = 0i32,
    ConnectionLatencyUpdated = 5i32,
    ConnectionRequest = 7i32,
    Disconnect = 1i32,
    Error = 4i32,
    MessageDelivered = 8i32,
    Receive = 2i32,
    ReceiveUnconnected = 3i32,
}
#[cfg(feature = "LiteNetLib+NetEvent+EType")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::NetEvent_EType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NetEvent/EType";
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
#[cfg(feature = "LiteNetLib+NetEvent+EType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::LiteNetLib::NetEvent_EType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LiteNetLib+NetEvent+EType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::LiteNetLib::NetEvent_EType {
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
#[cfg(feature = "LiteNetLib+NetEvent+EType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::LiteNetLib::NetEvent_EType {
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
#[cfg(feature = "LiteNetLib+NetEvent+EType")]
unsafe impl quest_hook::libil2cpp::Return for crate::LiteNetLib::NetEvent_EType {
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
