#[cfg(feature = "OculusLevelProductPacksSO")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductPacksSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _levelPackProductData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
    >,
    pub _levelPackRedirectionData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData,
            >,
        >,
    >,
}
#[cfg(feature = "OculusLevelProductPacksSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusLevelProductPacksSO => ""
    ."OculusLevelProductPacksSO"
);
#[cfg(feature = "OculusLevelProductPacksSO")]
impl std::ops::Deref for crate::GlobalNamespace::OculusLevelProductPacksSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductPacksSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusLevelProductPacksSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductPacksSO")]
impl crate::GlobalNamespace::OculusLevelProductPacksSO {
    pub fn ILevelPackProductDataContainer_OculusLevelProductCollectionModel_LevelPackProductData_OculusLevelProductCollectionModel_LevelProductData__SetLevelPackProductData(
        &mut self,
        newLevelPackProductData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ILevelPackProductDataContainer<OculusLevelProductCollectionModel.LevelPackProductData,OculusLevelProductCollectionModel.LevelProductData>.SetLevelPackProductData",
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
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
        > = __cordl_object.invoke("get_levelPackProductData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelPackRedirectionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData,
                >,
            >,
        > = __cordl_object.invoke("get_levelPackRedirectionData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusLevelProductPacksSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusLevelProductPacksSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusLevelProductPacksSO")]
impl AsRef<
    crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::OculusLevelProductPacksSO {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusLevelProductPacksSO")]
impl AsMut<
    crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::OculusLevelProductPacksSO {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
