#[cfg(feature = "SteamLevelProductCollectionModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductCollectionModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _levelIdToProductData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
            >,
        >,
    >,
    pub _levelPackIdToProductData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
            >,
        >,
    >,
    pub _levelPackRedirectionData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
            >,
        >,
    >,
    pub _maxPossibleInstalledDepots: i32,
}
#[cfg(feature = "SteamLevelProductCollectionModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SteamLevelProductCollectionModel => ""
    ."SteamLevelProductCollectionModel"
);
#[cfg(feature = "SteamLevelProductCollectionModel")]
impl std::ops::Deref for crate::GlobalNamespace::SteamLevelProductCollectionModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::SteamLevelProductCollectionModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel")]
impl crate::GlobalNamespace::SteamLevelProductCollectionModel {
    #[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
    pub type LevelDepotData = crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData;
    #[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
    pub type LevelPackProductData = crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData;
    #[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
    pub type LevelPackRedirectionData = crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData;
    #[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
    pub type LevelProductData = crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData;
    pub fn GetLevelPackProductData(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        > = __cordl_object.invoke("GetLevelPackProductData", (levelPackId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelPackRedirectionData(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
        > = __cordl_object.invoke("GetLevelPackRedirectionData", (levelPackId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelProductData(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        > = __cordl_object.invoke("GetLevelProductData", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        levelProductsSOs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductPacksSO,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelProductsSOs))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelProductsSOs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductPacksSO,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelProductsSOs))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelProductDataCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_levelProductDataCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelProductsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
                >,
            >,
        > = __cordl_object.invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamLevelProductCollectionModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductCollectionModel_LevelDepotData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub checkDepots: bool,
    pub noEnvironmentKeywordsDepotId: u32,
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData => ""
    ."SteamLevelProductCollectionModel/LevelDepotData"
);
#[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
impl std::ops::Deref
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
impl crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData {
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
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductCollectionModel_LevelPackProductData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _bundleId: u32,
    pub _levelProductsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
            >,
        >,
    >,
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData => ""
    ."SteamLevelProductCollectionModel/LevelPackProductData"
);
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
impl crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData {
    pub fn New(
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bundleId: u32,
        levelProducts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelPackId, bundleId, levelProducts))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bundleId: u32,
        levelProducts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelPackId, bundleId, levelProducts))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bundleId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_bundleId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelPackId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelProductsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
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
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
                >,
            >,
        > = __cordl_object.invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
impl AsRef<
    crate::GlobalNamespace::ILevelPackProductData_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ILevelPackProductData_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
impl AsMut<
    crate::GlobalNamespace::ILevelPackProductData_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ILevelPackProductData_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductCollectionModel_LevelPackRedirectionData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _targetLevelPackId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _shouldOwnLevelPackId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _redirectedBundleId: u32,
    pub _validUntilDate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData => ""
    ."SteamLevelProductCollectionModel/LevelPackRedirectionData"
);
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
impl std::ops::Deref
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
impl crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData {
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
    pub fn get_redirectedBundleId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_redirectedBundleId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shouldOwnLevelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_shouldOwnLevelPackId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetLevelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_targetLevelPackId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_validUntilDate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_validUntilDate", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductCollectionModel_LevelProductData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _appId: u32,
    pub _levelDepotData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData,
    >,
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData => ""
    ."SteamLevelProductCollectionModel/LevelProductData"
);
#[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
impl crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData {
    pub fn New(
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        appId: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelId, appId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        appId: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelId, appId))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_appId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_appId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelDepotData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData,
        > = __cordl_object.invoke("get_levelDepotData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
impl AsRef<crate::GlobalNamespace::ILevelProductData>
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILevelProductData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
impl AsMut<crate::GlobalNamespace::ILevelProductData>
for crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILevelProductData {
        unsafe { std::mem::transmute(self) }
    }
}
