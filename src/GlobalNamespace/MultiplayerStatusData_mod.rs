#[cfg(feature = "MultiplayerStatusData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerStatusData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub minimum_app_version: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub status: crate::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus,
    pub maintenance_start_time: i64,
    pub maintenance_end_time: i64,
    pub user_message: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerStatusData_UserMessage,
    >,
    pub use_gamelift: bool,
    pub use_xplatform_auth: bool,
}
#[cfg(feature = "MultiplayerStatusData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerStatusData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerStatusData";
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
#[cfg(feature = "MultiplayerStatusData")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerStatusData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusData")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerStatusData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusData")]
impl crate::GlobalNamespace::MultiplayerStatusData {
    #[cfg(feature = "MultiplayerStatusData+AvailabilityStatus")]
    pub type AvailabilityStatus = crate::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus;
    #[cfg(feature = "MultiplayerStatusData+UserMessage")]
    pub type UserMessage = crate::GlobalNamespace::MultiplayerStatusData_UserMessage;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_maintenanceEndTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_maintenanceEndTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maintenanceStartTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_maintenanceStartTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minimumAppVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_minimumAppVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useGamelift(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useGamelift", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useXPlatformAuth(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useXPlatformAuth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerStatusData_UserMessage,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerStatusData_UserMessage,
        > = __cordl_object.invoke("get_userMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maintenanceEndTime(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maintenanceEndTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maintenanceStartTime(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maintenanceStartTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minimumAppVersion(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minimumAppVersion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useGamelift(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useGamelift", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useXPlatformAuth(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useXPlatformAuth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_userMessage(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerStatusData_UserMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userMessage", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerStatusData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerStatusData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerStatusData+AvailabilityStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerStatusData_AvailabilityStatus {
    #[default]
    MaintenanceUpcoming = 1i32,
    Offline = 2i32,
    Online = 0i32,
}
#[cfg(feature = "MultiplayerStatusData+AvailabilityStatus")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AvailabilityStatus";
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
#[cfg(feature = "MultiplayerStatusData+AvailabilityStatus")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MultiplayerStatusData+AvailabilityStatus")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus {
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
#[cfg(feature = "MultiplayerStatusData+AvailabilityStatus")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus {
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
#[cfg(feature = "MultiplayerStatusData+AvailabilityStatus")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus {
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
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerStatusData_UserMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub localizations: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage,
            >,
        >,
    >,
}
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerStatusData_UserMessage {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerStatusData/UserMessage";
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
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerStatusData_UserMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerStatusData_UserMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
impl crate::GlobalNamespace::MultiplayerStatusData_UserMessage {
    #[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
    pub type LocalizedMessage = crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerStatusData_UserMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct UserMessage_MultiplayerStatusData_LocalizedMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub language: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerStatusData/UserMessage/LocalizedMessage";
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
#[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
impl std::ops::Deref
for crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
impl std::ops::DerefMut
for crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
impl crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
