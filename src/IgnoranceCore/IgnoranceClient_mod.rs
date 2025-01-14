#[cfg(feature = "IgnoranceCore+IgnoranceClient")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceClient {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ConnectAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ConnectPort: i32,
    pub ExpectedChannels: i32,
    pub PollTime: i32,
    pub MaximumPacketSize: i32,
    pub Verbosity: i32,
    pub IncomingOutgoingBufferSize: i32,
    pub ConnectionEventBufferSize: i32,
    pub UseSsl: bool,
    pub ValidateCertificate: bool,
    pub RootCertificatePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub RootCertificate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Incoming: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<
            crate::IgnoranceCore::IgnoranceIncomingPacket,
        >,
    >,
    pub Outgoing: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<
            crate::IgnoranceCore::IgnoranceOutgoingPacket,
        >,
    >,
    pub Commands: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<
            crate::IgnoranceCore::IgnoranceCommandPacket,
        >,
    >,
    pub ConnectionEvents: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<
            crate::IgnoranceCore::IgnoranceConnectionEvent,
        >,
    >,
    pub DisconnectionEvents: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<
            crate::IgnoranceCore::IgnoranceConnectionEvent,
        >,
    >,
    pub StatusUpdates: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<
            crate::IgnoranceCore::IgnoranceClientStats,
        >,
    >,
    pub CeaseOperation: bool,
    pub WorkerThread: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient")]
unsafe impl quest_hook::libil2cpp::Type for crate::IgnoranceCore::IgnoranceClient {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "IgnoranceCore";
    const CLASS_NAME: &'static str = "IgnoranceClient";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("SetupRingBuffersIfNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetupRingBuffersIfNull", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Start", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Stop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Stop", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThreadWorker(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ThreadWorker")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThreadWorker", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parameters))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsAlive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsAlive", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IgnoranceClient_ThreadParamInfo {
    pub Channels: i32,
    pub PollTime: i32,
    pub Port: i32,
    pub PacketSizeLimit: i32,
    pub Verbosity: i32,
    pub Address: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub UseSsl: bool,
    pub ValidateCertificate: bool,
    pub RootCertificatePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub RootCertificate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::IgnoranceCore::IgnoranceClient_ThreadParamInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "IgnoranceCore";
    const CLASS_NAME: &'static str = "IgnoranceClient/ThreadParamInfo";
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
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::IgnoranceCore::IgnoranceClient_ThreadParamInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::IgnoranceCore::IgnoranceClient_ThreadParamInfo {
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
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::IgnoranceCore::IgnoranceClient_ThreadParamInfo {
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
#[cfg(feature = "IgnoranceCore+IgnoranceClient+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::IgnoranceCore::IgnoranceClient_ThreadParamInfo {
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
