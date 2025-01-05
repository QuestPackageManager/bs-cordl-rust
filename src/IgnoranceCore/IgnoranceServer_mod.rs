#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceServer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub BindAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub BindPort: i32,
    pub MaximumChannels: i32,
    pub MaximumPeers: i32,
    pub MaximumPacketSize: i32,
    pub PollTime: i32,
    pub Verbosity: i32,
    pub IncomingOutgoingBufferSize: i32,
    pub ConnectionEventBufferSize: i32,
    pub IsFruityDevice: bool,
    pub BindAllInterfaces: bool,
    pub UseSsl: bool,
    pub CertificatePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Certificate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PrivateKeyPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PrivateKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub CeaseOperation: bool,
    pub Incoming: quest_hook::libil2cpp::Gc<
        crate::IgnoranceCore::IgnoranceIncomingPacket,
    >,
    pub Outgoing: quest_hook::libil2cpp::Gc<
        crate::IgnoranceCore::IgnoranceOutgoingPacket,
    >,
    pub Commands: quest_hook::libil2cpp::Gc<
        crate::IgnoranceCore::IgnoranceCommandPacket,
    >,
    pub ConnectionEvents: quest_hook::libil2cpp::Gc<
        crate::IgnoranceCore::IgnoranceConnectionEvent,
    >,
    pub DisconnectionEvents: quest_hook::libil2cpp::Gc<
        crate::IgnoranceCore::IgnoranceConnectionEvent,
    >,
    pub StatusUpdates: quest_hook::libil2cpp::Gc<
        crate::IgnoranceCore::IgnoranceServerStats,
    >,
    pub RecycledServerStatBlocks: quest_hook::libil2cpp::Gc<
        crate::IgnoranceCore::IgnoranceServerStats,
    >,
    pub WorkerThread: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceServer =>
    "IgnoranceCore"."IgnoranceServer"
);
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
impl std::ops::Deref for crate::IgnoranceCore::IgnoranceServer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
impl std::ops::DerefMut for crate::IgnoranceCore::IgnoranceServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
impl crate::IgnoranceCore::IgnoranceServer {
    #[cfg(feature = "IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
    pub type ThreadParamInfo = crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetupRingBuffersIfNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupRingBuffersIfNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThreadWorker(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThreadWorker", (parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAlive", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
impl quest_hook::libil2cpp::ObjectType for crate::IgnoranceCore::IgnoranceServer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IgnoranceServer_ThreadParamInfo {
    pub IsFruityDevice: bool,
    pub BindAllInterfaces: bool,
    pub Channels: i32,
    pub Peers: i32,
    pub PollTime: i32,
    pub Port: i32,
    pub PacketSizeLimit: i32,
    pub Verbosity: i32,
    pub Address: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub UseSsl: bool,
    pub CertificatePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Certificate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PrivateKeyPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PrivateKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceServer_ThreadParamInfo
    => "IgnoranceCore"."IgnoranceServer/ThreadParamInfo"
);
#[cfg(feature = "IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
impl crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo {}
