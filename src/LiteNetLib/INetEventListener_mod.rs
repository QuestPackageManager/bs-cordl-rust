#[cfg(feature = "LiteNetLib+INetEventListener")]
#[repr(C)]
#[derive(Debug)]
pub struct INetEventListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+INetEventListener")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::INetEventListener {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "INetEventListener";
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
#[cfg(feature = "LiteNetLib+INetEventListener")]
impl std::ops::Deref for crate::LiteNetLib::INetEventListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+INetEventListener")]
impl std::ops::DerefMut for crate::LiteNetLib::INetEventListener {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+INetEventListener")]
impl crate::LiteNetLib::INetEventListener {
    pub fn OnConnectionRequest(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::LiteNetLib::ConnectionRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::ConnectionRequest,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnConnectionRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnConnectionRequest", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (request))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnNetworkError(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        socketError: crate::System::Net::Sockets::SocketError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                            crate::System::Net::Sockets::SocketError,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnNetworkError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnNetworkError", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (endPoint, socketError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnNetworkLatencyUpdate(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        latency: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnNetworkLatencyUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnNetworkLatencyUpdate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (peer, latency))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnNetworkReceive(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::NetPacketReader,
                            >,
                            crate::LiteNetLib::DeliveryMethod,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnNetworkReceive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnNetworkReceive", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (peer, reader, deliveryMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnNetworkReceiveUnconnected(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
        messageType: crate::LiteNetLib::UnconnectedMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::NetPacketReader,
                            >,
                            crate::LiteNetLib::UnconnectedMessageType,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("OnNetworkReceiveUnconnected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnNetworkReceiveUnconnected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (remoteEndPoint, reader, messageType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPeerConnected(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnPeerConnected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnPeerConnected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (peer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPeerDisconnected(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        disconnectInfo: crate::LiteNetLib::DisconnectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
                            crate::LiteNetLib::DisconnectInfo,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnPeerDisconnected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnPeerDisconnected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (peer, disconnectInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "LiteNetLib+INetEventListener")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::INetEventListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
