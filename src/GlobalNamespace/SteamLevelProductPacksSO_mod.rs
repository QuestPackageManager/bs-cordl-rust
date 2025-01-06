#[cfg(feature = "SteamLevelProductPacksSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductPacksSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _levelPackProductData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
    >,
    pub _levelPackRedirectionData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
            >,
        >,
    >,
}
#[cfg(feature = "SteamLevelProductPacksSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SteamLevelProductPacksSO => ""
    ."SteamLevelProductPacksSO"
);
#[cfg(feature = "SteamLevelProductPacksSO")]
impl std::ops::Deref for crate::GlobalNamespace::SteamLevelProductPacksSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SteamLevelProductPacksSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl crate::GlobalNamespace::SteamLevelProductPacksSO {
    pub fn ILevelPackProductDataContainer_SteamLevelProductCollectionModel_LevelPackProductData_SteamLevelProductCollectionModel_LevelProductData__SetLevelPackProductData(
        &mut self,
        newLevelPackProductData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ILevelPackProductDataContainer<SteamLevelProductCollectionModel.LevelPackProductData,SteamLevelProductCollectionModel.LevelProductData>.SetLevelPackProductData",
                (newLevelPackProductData),
            )?;
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
    pub fn get_levelPackProductData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        > = __cordl_object.invoke("get_levelPackProductData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelPackRedirectionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
                >,
            >,
        > = __cordl_object.invoke("get_levelPackRedirectionData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamLevelProductPacksSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl AsRef<
    crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::SteamLevelProductPacksSO {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl AsMut<
    crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::SteamLevelProductPacksSO {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
