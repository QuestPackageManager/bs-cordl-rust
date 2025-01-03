#[cfg(feature = "Unity+XR+Oculus+Development")]
#[repr(C)]
#[derive(Debug)]
pub struct Development {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Development =>
    "Unity.XR.Oculus"."Development"
);
#[cfg(feature = "Unity+XR+Oculus+Development")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Development {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Development {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
impl crate::Unity::XR::Oculus::Development {
    #[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
    pub type UserDeveloperModeSettingCache = crate::Unity::XR::Oculus::Development_UserDeveloperModeSettingCache;
    pub fn OverrideDeveloperModeStart() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverrideDeveloperModeStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OverrideDeveloperModeStop() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverrideDeveloperModeStop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetDeveloperMode(
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrySetDeveloperMode", (active))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Development {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Development_UserDeveloperModeSettingCache {
    NoUserSettingCached = 0i32,
    UserSettingFalse = 1i32,
    UserSettingTrue = 2i32,
}
#[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::XR::Oculus::Development_UserDeveloperModeSettingCache => "Unity.XR.Oculus"
    ."Development/UserDeveloperModeSettingCache"
);
