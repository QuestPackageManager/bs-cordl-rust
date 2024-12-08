#[cfg(feature = "SteamLevelProductPacksSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductPacksSO {
    __cordl_parent: PersistentScriptableObject,
    pub _levelPackProductData: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
    pub _levelPackRedirectionData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
    >,
}
#[cfg(feature = "SteamLevelProductPacksSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SteamLevelProductPacksSO => ""
    ."SteamLevelProductPacksSO"
);
#[cfg(feature = "SteamLevelProductPacksSO")]
impl std::ops::Deref for SteamLevelProductPacksSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl std::ops::DerefMut for SteamLevelProductPacksSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl SteamLevelProductPacksSO {
    pub fn get_levelPackRedirectionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
        > = __cordl_object.invoke("get_levelPackRedirectionData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ILevelPackProductDataContainer_SteamLevelProductCollectionModel_LevelPackProductData_SteamLevelProductCollectionModel_LevelProductData__SetLevelPackProductData(
        &mut self,
        newLevelPackProductData: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ILevelPackProductDataContainer<SteamLevelProductCollectionModel.LevelPackProductData,SteamLevelProductCollectionModel.LevelProductData>.SetLevelPackProductData",
                (newLevelPackProductData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_levelPackProductData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData = __cordl_object
            .invoke("get_levelPackProductData", ())?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl quest_hook::libil2cpp::ObjectType for SteamLevelProductPacksSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
