#[cfg(feature = "PS5SharedPackageSKUsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5SharedPackageSKUsSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _availableSKUs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PS5PublisherSKUSettingsSO,
    >,
    pub _conceptId: *mut crate::System::String,
    pub _buildType: crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType,
    pub _buildVersion: *mut crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
    pub _latestBuildVersion: *mut crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
}
#[cfg(feature = "PS5SharedPackageSKUsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PS5SharedPackageSKUsSO => ""
    ."PS5SharedPackageSKUsSO"
);
#[cfg(feature = "PS5SharedPackageSKUsSO")]
impl std::ops::Deref for crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO")]
impl crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    #[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
    pub type BuildType = crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType;
    #[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
    pub type PS5BuildVersion = crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion;
    pub fn GetPrimarySKU(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PS5PublisherSKUSettingsSO,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PS5PublisherSKUSettingsSO = __cordl_object
            .invoke("GetPrimarySKU", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn get_availableSKUs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PS5PublisherSKUSettingsSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PS5PublisherSKUSettingsSO,
        > = __cordl_object.invoke("get_availableSKUs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_buildType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType = __cordl_object
            .invoke("get_buildType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_buildVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion = __cordl_object
            .invoke("get_buildVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_conceptId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_conceptId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_latestBuildVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion = __cordl_object
            .invoke("get_latestBuildVersion", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PS5SharedPackageSKUsSO_BuildType {
    Application = 0i32,
    RemasterPatch = 1i32,
}
#[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType => ""
    ."PS5SharedPackageSKUsSO/BuildType"
);
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5SharedPackageSKUsSO_PS5BuildVersion {
    __cordl_parent: crate::System::Object,
    pub _masterVersion: *mut crate::System::String,
    pub _contentVersion: *mut crate::System::String,
}
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion => ""
    ."PS5SharedPackageSKUsSO/PS5BuildVersion"
);
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
impl std::ops::Deref for crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
impl crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
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
    pub fn get_contentVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_contentVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_masterVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_masterVersion", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
