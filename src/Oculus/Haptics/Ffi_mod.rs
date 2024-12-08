#[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ffi_Controller {
    Both = 2i32,
    Left = 0i32,
    Right = 1i32,
}
#[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Ffi_Controller =>
    "Oculus.Haptics"."Ffi/Controller"
);
#[cfg(feature = "Oculus+Haptics+Ffi")]
#[repr(C)]
#[derive(Debug)]
pub struct Ffi {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Haptics+Ffi")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Ffi => "Oculus.Haptics"."Ffi"
);
#[cfg(feature = "Oculus+Haptics+Ffi")]
impl std::ops::Deref for crate::Oculus::Haptics::Ffi {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi")]
impl std::ops::DerefMut for crate::Oculus::Haptics::Ffi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi")]
impl crate::Oculus::Haptics::Ffi {
    pub const InvalidId: i32 = -1i32;
    pub const NativeLibName: &'static str = "haptics_sdk";
    #[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
    pub type NullBackendStatistics = crate::Oculus::Haptics::Ffi_NullBackendStatistics;
    #[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
    pub type Controller = crate::Oculus::Haptics::Ffi_Controller;
    #[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
    pub type LogLevel = crate::Oculus::Haptics::Ffi_LogLevel;
    #[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
    pub type LogCallback = crate::Oculus::Haptics::Ffi_LogCallback;
    #[cfg(feature = "Oculus+Haptics+Ffi+Result")]
    pub type Result = crate::Oculus::Haptics::Ffi_Result;
    #[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
    pub type SdkVersion = crate::Oculus::Haptics::Ffi_SdkVersion;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "Oculus+Haptics+Ffi")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Haptics::Ffi {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct Ffi_LogCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Ffi_LogCallback =>
    "Oculus.Haptics"."Ffi/LogCallback"
);
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
impl std::ops::Deref for crate::Oculus::Haptics::Ffi_LogCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
impl std::ops::DerefMut for crate::Oculus::Haptics::Ffi_LogCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
impl crate::Oculus::Haptics::Ffi_LogCallback {
    pub fn BeginInvoke(
        &mut self,
        level: crate::Oculus::Haptics::Ffi_LogLevel,
        message: *mut crate::System::String,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (level, message, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        level: crate::Oculus::Haptics::Ffi_LogLevel,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (level, message))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Haptics::Ffi_LogCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ffi_LogLevel {
    Debug = 1i32,
    Error = 4i32,
    Info = 2i32,
    Trace = 0i32,
    Warn = 3i32,
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Ffi_LogLevel =>
    "Oculus.Haptics"."Ffi/LogLevel"
);
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ffi_NullBackendStatistics {
    pub play_call_count: i64,
    pub stop_call_count: i64,
    pub samples_played: i64,
}
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Ffi_NullBackendStatistics =>
    "Oculus.Haptics"."Ffi/NullBackendStatistics"
);
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Haptics::Ffi_NullBackendStatistics {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
impl crate::Oculus::Haptics::Ffi_NullBackendStatistics {}
#[cfg(feature = "Oculus+Haptics+Ffi+Result")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ffi_Result {
    ClipIdInvalid = -9i32,
    CreatePlayerFailed = -8i32,
    Error = -1i32,
    InstanceAlreadyInitialized = -3i32,
    InstanceAlreadyUninitialized = -4i32,
    InstanceInitializationFailed = -2i32,
    InstanceNotInitialized = -5i32,
    InvalidUtf8 = -6i32,
    LoadClipFailed = -7i32,
    NoClipLoaded = -14i32,
    PlayerIdInvalid = -10i32,
    PlayerInvalidAmplitude = -11i32,
    PlayerInvalidFrequencyShift = -12i32,
    PlayerInvalidPriority = -13i32,
    Success = 0i32,
}
#[cfg(feature = "Oculus+Haptics+Ffi+Result")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Ffi_Result => "Oculus.Haptics"
    ."Ffi/Result"
);
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ffi_SdkVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Haptics::Ffi_SdkVersion =>
    "Oculus.Haptics"."Ffi/SdkVersion"
);
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Haptics::Ffi_SdkVersion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
impl crate::Oculus::Haptics::Ffi_SdkVersion {}
