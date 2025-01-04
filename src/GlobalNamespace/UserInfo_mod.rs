#[cfg(feature = "UserInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct UserInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub platform: crate::GlobalNamespace::UserInfo_Platform,
    pub platformUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UserInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UserInfo => ""."UserInfo"
);
#[cfg(feature = "UserInfo")]
impl std::ops::Deref for crate::GlobalNamespace::UserInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UserInfo")]
impl std::ops::DerefMut for crate::GlobalNamespace::UserInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UserInfo")]
impl crate::GlobalNamespace::UserInfo {
    #[cfg(feature = "UserInfo+Platform")]
    pub type Platform = crate::GlobalNamespace::UserInfo_Platform;
    pub fn New(
        platform: crate::GlobalNamespace::UserInfo_Platform,
        platformUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platform, platformUserId, userName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        platform: crate::GlobalNamespace::UserInfo_Platform,
        platformUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platform, platformUserId, userName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UserInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UserInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UserInfo+Platform")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserInfo_Platform {
    #[default]
    Oculus = 2i32,
    PS4 = 3i32,
    PS5 = 4i32,
    Steam = 1i32,
    Test = 0i32,
}
#[cfg(feature = "UserInfo+Platform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UserInfo_Platform => ""
    ."UserInfo/Platform"
);
