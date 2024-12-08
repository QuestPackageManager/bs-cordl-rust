#[cfg(feature = "SonyLevelProductPackSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductPackSO {
    __cordl_parent: PersistentScriptableObject,
    pub _levelPackProductData: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData,
    pub _levelPackRedirectionData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData,
    >,
}
#[cfg(feature = "SonyLevelProductPackSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SonyLevelProductPackSO => ""."SonyLevelProductPackSO"
);
#[cfg(feature = "SonyLevelProductPackSO")]
impl std::ops::Deref for SonyLevelProductPackSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSO")]
impl std::ops::DerefMut for SonyLevelProductPackSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSO")]
impl SonyLevelProductPackSO {
    pub fn set_levelPackProductData(
        &mut self,
        value: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelPackProductData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_levelPackProductData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData = __cordl_object
            .invoke("get_levelPackProductData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelPackRedirectionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData,
        > = __cordl_object.invoke("get_levelPackRedirectionData", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_levelPackRedirectionData(
        &mut self,
        value: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelPackRedirectionData", (value))?;
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
#[cfg(feature = "SonyLevelProductPackSO")]
impl quest_hook::libil2cpp::ObjectType for SonyLevelProductPackSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
