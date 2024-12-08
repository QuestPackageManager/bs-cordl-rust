#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceServer {
    __cordl_parent: crate::System::Object,
    pub BindAddress: *mut crate::System::String,
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
    pub CertificatePath: *mut crate::System::String,
    pub Certificate: *mut crate::System::String,
    pub PrivateKeyPath: *mut crate::System::String,
    pub PrivateKey: *mut crate::System::String,
    pub CeaseOperation: bool,
    pub Incoming: *mut crate::IgnoranceThirdparty::RingBuffer_1<
        crate::IgnoranceCore::IgnoranceIncomingPacket,
    >,
    pub Outgoing: *mut crate::IgnoranceThirdparty::RingBuffer_1<
        crate::IgnoranceCore::IgnoranceOutgoingPacket,
    >,
    pub Commands: *mut crate::IgnoranceThirdparty::RingBuffer_1<
        crate::IgnoranceCore::IgnoranceCommandPacket,
    >,
    pub ConnectionEvents: *mut crate::IgnoranceThirdparty::RingBuffer_1<
        crate::IgnoranceCore::IgnoranceConnectionEvent,
    >,
    pub DisconnectionEvents: *mut crate::IgnoranceThirdparty::RingBuffer_1<
        crate::IgnoranceCore::IgnoranceConnectionEvent,
    >,
    pub StatusUpdates: *mut crate::IgnoranceThirdparty::RingBuffer_1<
        crate::IgnoranceCore::IgnoranceServerStats,
    >,
    pub RecycledServerStatBlocks: *mut crate::IgnoranceThirdparty::RingBuffer_1<
        crate::IgnoranceCore::IgnoranceServerStats,
    >,
    pub WorkerThread: *mut crate::System::Threading::Thread,
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceServer =>
    "IgnoranceCore"."IgnoranceServer"
);
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
impl std::ops::Deref for crate::IgnoranceCore::IgnoranceServer {
    type Target = crate::System::Object;
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupRingBuffersIfNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupRingBuffersIfNull", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThreadWorker(
        &mut self,
        parameters: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThreadWorker", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAlive", ())?;
        Ok(__cordl_ret)
    }
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[derive(Debug, Clone)]
pub struct IgnoranceServer_ThreadParamInfo {
    pub IsFruityDevice: bool,
    pub BindAllInterfaces: bool,
    pub Channels: i32,
    pub Peers: i32,
    pub PollTime: i32,
    pub Port: i32,
    pub PacketSizeLimit: i32,
    pub Verbosity: i32,
    pub Address: *mut crate::System::String,
    pub UseSsl: bool,
    pub CertificatePath: *mut crate::System::String,
    pub Certificate: *mut crate::System::String,
    pub PrivateKeyPath: *mut crate::System::String,
    pub PrivateKey: *mut crate::System::String,
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
