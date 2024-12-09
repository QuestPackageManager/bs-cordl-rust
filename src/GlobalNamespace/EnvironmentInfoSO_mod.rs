#[cfg(feature = "EnvironmentInfoSO")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentInfoSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _environmentName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _colorScheme: *mut crate::GlobalNamespace::ColorSchemeSO,
    pub _sceneInfo: *mut crate::GlobalNamespace::SceneInfo,
    pub _serializedName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _tags: crate::GlobalNamespace::EnvironmentInfoSO_Tags,
    pub _environmentType: crate::GlobalNamespace::EnvironmentType,
    pub _environmentSizeData: *mut crate::GlobalNamespace::EnvironmentSizeData,
    pub _environmentIntensityReductionOptions: *mut crate::GlobalNamespace::EnvironmentIntensityReductionOptions,
    pub _environmentKeywords: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _lightGroups: *mut crate::GlobalNamespace::EnvironmentLightGroups,
    pub _defaultLightshowAsset: *mut crate::UnityEngine::TextAsset,
    pub _order: i32,
}
#[cfg(feature = "EnvironmentInfoSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentInfoSO => ""
    ."EnvironmentInfoSO"
);
#[cfg(feature = "EnvironmentInfoSO")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentInfoSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentInfoSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentInfoSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentInfoSO")]
impl crate::GlobalNamespace::EnvironmentInfoSO {
    pub const kLightGroupSubDir: &'static str = "LightGroups";
    #[cfg(feature = "EnvironmentInfoSO+Tags")]
    pub type Tags = crate::GlobalNamespace::EnvironmentInfoSO_Tags;
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
    pub fn get_colorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ColorSchemeSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ColorSchemeSO = __cordl_object
            .invoke("get_colorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultLightshowAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TextAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextAsset = __cordl_object
            .invoke("get_defaultLightshowAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentIntensityReductionOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::EnvironmentIntensityReductionOptions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentIntensityReductionOptions = __cordl_object
            .invoke("get_environmentIntensityReductionOptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentKeywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_environmentKeywords", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentLightGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::IEnvironmentLightGroups,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IEnvironmentLightGroups = __cordl_object
            .invoke("get_environmentLightGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_environmentName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentSizeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::EnvironmentSizeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentSizeData = __cordl_object
            .invoke("get_environmentSizeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentType = __cordl_object
            .invoke("get_environmentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isBranded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isBranded", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInDevelopment(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInDevelopment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_order(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_order", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sceneInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::SceneInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SceneInfo = __cordl_object
            .invoke("get_sceneInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_serializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_serializedName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "EnvironmentInfoSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::EnvironmentInfoSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentInfoSO+Tags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentInfoSO_Tags {
    Branded = 1i32,
    InDevelopment = 2i32,
}
#[cfg(feature = "EnvironmentInfoSO+Tags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentInfoSO_Tags => ""
    ."EnvironmentInfoSO/Tags"
);
