#[cfg(feature = "IgnoranceCore+IgnoranceClient")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceClient {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ConnectAddress: *mut quest_hook::libil2cpp::Il2CppString,
    pub ConnectPort: i32,
    pub ExpectedChannels: i32,
    pub PollTime: i32,
    pub MaximumPacketSize: i32,
    pub Verbosity: i32,
    pub IncomingOutgoingBufferSize: i32,
    pub ConnectionEventBufferSize: i32,
    pub UseSsl: bool,
    pub ValidateCertificate: bool,
    pub RootCertificatePath: *mut quest_hook::libil2cpp::Il2CppString,
    pub RootCertificate: *mut quest_hook::libil2cpp::Il2CppString,
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
        crate::IgnoranceCore::IgnoranceClientStats,
    >,
    pub CeaseOperation: bool,
    pub WorkerThread: *mut crate::System::Threading::Thread,
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceClient =>
    "IgnoranceCore"."IgnoranceClient"
);
#[cfg(feature = "IgnoranceCore+IgnoranceClient")]
impl std::ops::Deref for crate::IgnoranceCore::IgnoranceClient {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient")]
impl std::ops::DerefMut for crate::IgnoranceCore::IgnoranceClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient")]
impl crate::IgnoranceCore::IgnoranceClient {
    #[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
    pub type ThreadParamInfo = crate::IgnoranceCore::IgnoranceClient_ThreadParamInfo;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn ThreadWorker(
        &mut self,
        parameters: *mut quest_hook::libil2cpp::Il2CppObject,
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
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient")]
impl quest_hook::libil2cpp::ObjectType for crate::IgnoranceCore::IgnoranceClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IgnoranceClient_ThreadParamInfo {
    pub Channels: i32,
    pub PollTime: i32,
    pub Port: i32,
    pub PacketSizeLimit: i32,
    pub Verbosity: i32,
    pub Address: *mut quest_hook::libil2cpp::Il2CppString,
    pub UseSsl: bool,
    pub ValidateCertificate: bool,
    pub RootCertificatePath: *mut quest_hook::libil2cpp::Il2CppString,
    pub RootCertificate: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceClient_ThreadParamInfo
    => "IgnoranceCore"."IgnoranceClient/ThreadParamInfo"
);
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::IgnoranceCore::IgnoranceClient_ThreadParamInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
impl crate::IgnoranceCore::IgnoranceClient_ThreadParamInfo {}
