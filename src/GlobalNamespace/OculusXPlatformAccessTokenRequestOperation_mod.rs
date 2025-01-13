#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusXPlatformAccessTokenRequestOperation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tokenData: crate::GlobalNamespace::XPlatformAccessTokenData,
    pub _operationState: crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState,
}
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusXPlatformAccessTokenRequestOperation";
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
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
impl std::ops::Deref
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
impl crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation {
    pub const kMaxTokenRetry: i32 = 3i32;
    pub const kMillisecondsDelayToCheckCallbackResponse: i32 = 500i32;
    #[cfg(
        feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
    )]
    pub type OculusTokenRequestOperationState = crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCompleteLoadingOculusAccessToken(
        &mut self,
        message: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleteLoadingOculusAccessToken", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestAccessToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestAccessToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        > = __cordl_object.invoke("RequestXPlatformAccessToken", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Run", (cancellationToken))?;
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
}
#[cfg(feature = "OculusXPlatformAccessTokenRequestOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState {
    #[default]
    Failed = 3i32,
    NotStarted = 0i32,
    Requesting = 1i32,
    Succeeded = 2i32,
}
#[cfg(
    feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusXPlatformAccessTokenRequestOperation/OculusTokenRequestOperationState";
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
#[cfg(
    feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState {
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
#[cfg(
    feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState {
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
#[cfg(
    feature = "OculusXPlatformAccessTokenRequestOperation+OculusTokenRequestOperationState"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OculusXPlatformAccessTokenRequestOperation_OculusTokenRequestOperationState {
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
