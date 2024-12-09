#[cfg(feature = "PS4PublisherSKUSettingsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PS4PublisherSKUSettingsSO {
    __cordl_parent: crate::GlobalNamespace::SonyPublisherSKUSettingsSO,
    pub _buildVersion: *mut crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
    pub _latestBuildVersion: *mut crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
    pub _parentalLockLevel: i32,
    pub _npTitleFilenamePath: *mut quest_hook::libil2cpp::Il2CppString,
    pub _defaultAgeRestriction: i32,
    pub _applicationCategory: crate::GlobalNamespace::PS4ApplicationCategory,
}
#[cfg(feature = "PS4PublisherSKUSettingsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PS4PublisherSKUSettingsSO => ""
    ."PS4PublisherSKUSettingsSO"
);
#[cfg(feature = "PS4PublisherSKUSettingsSO")]
impl std::ops::Deref for crate::GlobalNamespace::PS4PublisherSKUSettingsSO {
    type Target = crate::GlobalNamespace::SonyPublisherSKUSettingsSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS4PublisherSKUSettingsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS4PublisherSKUSettingsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS4PublisherSKUSettingsSO")]
impl crate::GlobalNamespace::PS4PublisherSKUSettingsSO {
    #[cfg(feature = "PS4PublisherSKUSettingsSO+PS4BuildVersion")]
    pub type PS4BuildVersion = crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion;
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
    pub fn get_applicationCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PS4ApplicationCategory> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PS4ApplicationCategory = __cordl_object
            .invoke("get_applicationCategory", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_buildVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion = __cordl_object
            .invoke("get_buildVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultAgeRestriction(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_defaultAgeRestriction", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_latestBuildVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion = __cordl_object
            .invoke("get_latestBuildVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_npTitleFilenamePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_npTitleFilenamePath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_parentalLockLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_parentalLockLevel", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PS4PublisherSKUSettingsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS4PublisherSKUSettingsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PS4PublisherSKUSettingsSO+PS4BuildVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct PS4PublisherSKUSettingsSO_PS4BuildVersion {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _masterVersion: *mut quest_hook::libil2cpp::Il2CppString,
    pub _applicationVersion: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "PS4PublisherSKUSettingsSO+PS4BuildVersion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion => ""
    ."PS4PublisherSKUSettingsSO/PS4BuildVersion"
);
#[cfg(feature = "PS4PublisherSKUSettingsSO+PS4BuildVersion")]
impl std::ops::Deref
for crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS4PublisherSKUSettingsSO+PS4BuildVersion")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS4PublisherSKUSettingsSO+PS4BuildVersion")]
impl crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion {
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
    pub fn get_applicationVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_applicationVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_masterVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_masterVersion", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PS4PublisherSKUSettingsSO+PS4BuildVersion")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
