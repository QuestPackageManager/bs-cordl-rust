#[cfg(feature = "PS4PublisherSKUSettingsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PS4PublisherSKUSettingsSO {
    __cordl_parent: crate::GlobalNamespace::SonyPublisherSKUSettingsSO,
    pub _buildVersion: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
    >,
    pub _latestBuildVersion: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
    >,
    pub _parentalLockLevel: i32,
    pub _npTitleFilenamePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
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
    pub fn get_applicationCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PS4ApplicationCategory> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PS4ApplicationCategory = __cordl_object
            .invoke("get_applicationCategory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_buildVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
        > = __cordl_object.invoke("get_buildVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultAgeRestriction(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_defaultAgeRestriction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_latestBuildVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
        > = __cordl_object.invoke("get_latestBuildVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_npTitleFilenamePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_npTitleFilenamePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parentalLockLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_parentalLockLevel", ())?;
        Ok(__cordl_ret.into())
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
    pub _masterVersion: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyVersion>,
    pub _applicationVersion: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyVersion,
    >,
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
    pub fn CopyValueFrom(
        &mut self,
        newVersion: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS4PublisherSKUSettingsSO_PS4BuildVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValueFrom", (newVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncreaseAppVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseAppVersion", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_applicationVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyVersion,
        > = __cordl_object.invoke("get_applicationVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_masterVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyVersion,
        > = __cordl_object.invoke("get_masterVersion", ())?;
        Ok(__cordl_ret.into())
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
