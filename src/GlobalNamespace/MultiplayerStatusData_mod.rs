#[cfg(feature = "MultiplayerStatusData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerStatusData {
    __cordl_parent: crate::System::Object,
    pub minimum_app_version: *mut crate::System::String,
    pub status: crate::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus,
    pub maintenance_start_time: i64,
    pub maintenance_end_time: i64,
    pub user_message: *mut crate::GlobalNamespace::MultiplayerStatusData_UserMessage,
    pub use_gamelift: bool,
    pub use_xplatform_auth: bool,
}
#[cfg(feature = "MultiplayerStatusData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerStatusData => ""
    ."MultiplayerStatusData"
);
#[cfg(feature = "MultiplayerStatusData")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerStatusData {
    type Target = crate::System::Object;
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
    pub fn get_maintenanceEndTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_maintenanceEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maintenanceStartTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_maintenanceStartTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minimumAppVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_minimumAppVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useGamelift(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useGamelift", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useXPlatformAuth(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useXPlatformAuth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerStatusData_UserMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerStatusData_UserMessage = __cordl_object
            .invoke("get_userMessage", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_minimumAppVersion(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minimumAppVersion", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_userMessage(
        &mut self,
        value: *mut crate::GlobalNamespace::MultiplayerStatusData_UserMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userMessage", (value))?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerStatusData_AvailabilityStatus {
    MaintenanceUpcoming = 1i32,
    Offline = 2i32,
    Online = 0i32,
}
#[cfg(feature = "MultiplayerStatusData+AvailabilityStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerStatusData_AvailabilityStatus => ""
    ."MultiplayerStatusData/AvailabilityStatus"
);
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerStatusData_UserMessage {
    __cordl_parent: crate::System::Object,
    pub localizations: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage,
    >,
}
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerStatusData_UserMessage => ""
    ."MultiplayerStatusData/UserMessage"
);
#[cfg(feature = "MultiplayerStatusData+UserMessage")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerStatusData_UserMessage {
    type Target = crate::System::Object;
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
    __cordl_parent: crate::System::Object,
    pub language: *mut crate::System::String,
    pub message: *mut crate::System::String,
}
#[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage => ""
    ."MultiplayerStatusData/UserMessage/LocalizedMessage"
);
#[cfg(feature = "MultiplayerStatusData+UserMessage+LocalizedMessage")]
impl std::ops::Deref
for crate::GlobalNamespace::UserMessage_MultiplayerStatusData_LocalizedMessage {
    type Target = crate::System::Object;
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
