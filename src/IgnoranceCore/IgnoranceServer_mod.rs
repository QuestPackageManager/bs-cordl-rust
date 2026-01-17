#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct IgnoranceServer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
        crate::IgnoranceThirdparty::RingBuffer_1<crate::IgnoranceCore::IgnoranceIncomingPacket>,
    >,
    pub Outgoing: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<crate::IgnoranceCore::IgnoranceOutgoingPacket>,
    >,
    pub Commands: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<crate::IgnoranceCore::IgnoranceCommandPacket>,
    >,
    pub ConnectionEvents: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<crate::IgnoranceCore::IgnoranceConnectionEvent>,
    >,
    pub DisconnectionEvents: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<crate::IgnoranceCore::IgnoranceConnectionEvent>,
    >,
    pub StatusUpdates: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<crate::IgnoranceCore::IgnoranceServerStats>,
    >,
    pub RecycledServerStatBlocks: quest_hook::libil2cpp::Gc<
        crate::IgnoranceThirdparty::RingBuffer_1<crate::IgnoranceCore::IgnoranceServerStats>,
    >,
    pub WorkerThread: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
}
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer")]
unsafe impl quest_hook::libil2cpp::Type for crate::IgnoranceCore::IgnoranceServer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "IgnoranceCore";
    const CLASS_NAME: &'static str = "IgnoranceServer";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
impl std::ops::Deref for crate::IgnoranceCore::IgnoranceServer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
impl std::ops::DerefMut for crate::IgnoranceCore::IgnoranceServer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceServer")]
impl crate::IgnoranceCore::IgnoranceServer {
    #[cfg(feature = "IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
    pub type ThreadParamInfo = crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetupRingBuffersIfNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "SetupRingBuffersIfNull",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupRingBuffersIfNull",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Start(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Start",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Stop(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Stop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Stop",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ThreadWorker(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ThreadWorker")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ThreadWorker", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parameters))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsAlive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsAlive",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer")]
impl quest_hook::libil2cpp::ObjectType for crate::IgnoranceCore::IgnoranceServer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
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
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "IgnoranceCore";
    const CLASS_NAME: &'static str = "IgnoranceServer/ThreadParamInfo";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo
{
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
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo
{
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
#[cfg(feature = "cordl_class_IgnoranceCore+IgnoranceServer+ThreadParamInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::IgnoranceCore::IgnoranceServer_ThreadParamInfo
{
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
