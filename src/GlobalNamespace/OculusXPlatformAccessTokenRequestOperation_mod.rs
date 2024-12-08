#[cfg(
    feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState {
    Failed = 3i32,
    NotStarted = 0i32,
    Requesting = 1i32,
    Succeeded = 2i32,
}
#[cfg(
    feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState
    => ""."OculusXPlatformAccessTokenRequestOperation/OculusTokenRequestOperationState"
);
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusXPlatformAccessTokenRequestOperation {
    __cordl_parent: crate::System::Object,
    pub _tokenData: XPlatformAccessTokenData,
    pub _operationState: crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState,
}
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusXPlatformAccessTokenRequestOperation => ""
    ."OculusXPlatformAccessTokenRequestOperation"
);
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
impl std::ops::Deref for OculusXPlatformAccessTokenRequestOperation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
impl std::ops::DerefMut for OculusXPlatformAccessTokenRequestOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
impl OculusXPlatformAccessTokenRequestOperation {
    pub const kMaxTokenRetry: i32 = 3i32;
    pub const kMillisecondsDelayToCheckCallbackResponse: i32 = 500i32;
    #[cfg(
        feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
    )]
    pub type OculusTokenRequestOperationState = crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState;
    #[cfg(
        feature = "OculusXPlatformAccessTokenRequestOperation+_RequestXPlatformAccessToken_d__7"
    )]
    pub type _RequestXPlatformAccessToken_d__7 = crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation__RequestXPlatformAccessToken_d__7;
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
    pub fn RequestXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<XPlatformAccessTokenData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            XPlatformAccessTokenData,
        > = __cordl_object.invoke("RequestXPlatformAccessToken", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn RequestAccessToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestAccessToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnCompleteLoadingOculusAccessToken(
        &mut self,
        message: *mut crate::Oculus::Platform::Message_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleteLoadingOculusAccessToken", (message))?;
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
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
impl quest_hook::libil2cpp::ObjectType for OculusXPlatformAccessTokenRequestOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
