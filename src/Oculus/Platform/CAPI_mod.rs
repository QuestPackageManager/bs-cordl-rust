#[cfg(feature = "Oculus+Platform+CAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct CAPI {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+CAPI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::CAPI => "Oculus.Platform"
    ."CAPI"
);
#[cfg(feature = "Oculus+Platform+CAPI")]
impl std::ops::Deref for crate::Oculus::Platform::CAPI {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI")]
impl std::ops::DerefMut for crate::Oculus::Platform::CAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI")]
impl crate::Oculus::Platform::CAPI {
    pub const DLL_NAME: &'static str = "ovrplatformloader";
    pub const VoipFilterBufferSize: i32 = 480i32;
    #[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
    pub type ovrNetSyncVec3 = crate::Oculus::Platform::CAPI_ovrNetSyncVec3;
    #[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
    pub type OculusInitParams = crate::Oculus::Platform::CAPI_OculusInitParams;
    #[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
    pub type FilterCallback = crate::Oculus::Platform::CAPI_FilterCallback;
    #[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
    pub type ovrKeyValuePair = crate::Oculus::Platform::CAPI_ovrKeyValuePair;
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
#[cfg(feature = "Oculus+Platform+CAPI")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::CAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct CAPI_FilterCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::CAPI_FilterCallback =>
    "Oculus.Platform"."CAPI/FilterCallback"
);
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
impl std::ops::Deref for crate::Oculus::Platform::CAPI_FilterCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
impl std::ops::DerefMut for crate::Oculus::Platform::CAPI_FilterCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
impl crate::Oculus::Platform::CAPI_FilterCallback {
    pub fn BeginInvoke(
        &mut self,
        pcmData: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i16>,
        >,
        pcmDataLength: crate::System::UIntPtr,
        frequency: i32,
        numChannels: i32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pcmData, pcmDataLength, frequency, numChannels, callback, object),
            )?;
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
        pcmData: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i16>,
        >,
        pcmDataLength: crate::System::UIntPtr,
        frequency: i32,
        numChannels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pcmData, pcmDataLength, frequency, numChannels))?;
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
#[cfg(feature = "Oculus+Platform+CAPI+FilterCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::CAPI_FilterCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CAPI_OculusInitParams {
    pub sType: i32,
    pub email: *mut crate::System::String,
    pub password: *mut crate::System::String,
    pub appId: u64,
    pub uriPrefixOverride: *mut crate::System::String,
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::CAPI_OculusInitParams =>
    "Oculus.Platform"."CAPI/OculusInitParams"
);
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Platform::CAPI_OculusInitParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+OculusInitParams")]
impl crate::Oculus::Platform::CAPI_OculusInitParams {}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CAPI_ovrKeyValuePair {
    pub key_: *mut crate::System::String,
    pub valueType_: crate::Oculus::Platform::KeyValuePairType,
    pub stringValue_: *mut crate::System::String,
    pub intValue_: i32,
    pub doubleValue_: f64,
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::CAPI_ovrKeyValuePair =>
    "Oculus.Platform"."CAPI/ovrKeyValuePair"
);
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Platform::CAPI_ovrKeyValuePair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrKeyValuePair")]
impl crate::Oculus::Platform::CAPI_ovrKeyValuePair {
    pub fn _ctor_String0(
        &mut self,
        key: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (key, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f64_2(
        &mut self,
        key: *mut crate::System::String,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (key, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        key: *mut crate::System::String,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (key, value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CAPI_ovrNetSyncVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::CAPI_ovrNetSyncVec3 =>
    "Oculus.Platform"."CAPI/ovrNetSyncVec3"
);
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Platform+CAPI+ovrNetSyncVec3")]
impl crate::Oculus::Platform::CAPI_ovrNetSyncVec3 {}
