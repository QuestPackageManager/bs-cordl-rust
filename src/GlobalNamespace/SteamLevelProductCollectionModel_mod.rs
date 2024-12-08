#[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductCollectionModel_LevelDepotData {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    __cordl_parent: crate::System::Object,
    pub _levelPackId: *mut crate::System::String,
    pub _bundleId: u32,
    pub _levelProductsData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
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
    type Target = crate::System::Object;
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
        levelPackId: *mut crate::System::String,
        bundleId: u32,
        levelProducts: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelPackId, bundleId, levelProducts))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        levelPackId: *mut crate::System::String,
        bundleId: u32,
        levelProducts: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelPackId, bundleId, levelProducts))?;
        Ok(__cordl_ret)
    }
    pub fn get_bundleId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_bundleId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_levelPackId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelProductsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        > = __cordl_object.invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductCollectionModel_LevelPackRedirectionData {
    __cordl_parent: crate::System::Object,
    pub _targetLevelPackId: *mut crate::System::String,
    pub _shouldOwnLevelPackId: *mut crate::System::String,
    pub _redirectedBundleId: u32,
    pub _validUntilDate: *mut crate::System::String,
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
    type Target = crate::System::Object;
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
    pub fn get_redirectedBundleId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_redirectedBundleId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shouldOwnLevelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_shouldOwnLevelPackId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetLevelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_targetLevelPackId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_validUntilDate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_validUntilDate", ())?;
        Ok(__cordl_ret)
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
    __cordl_parent: crate::System::Object,
    pub _levelId: *mut crate::System::String,
    pub _appId: u32,
    pub _levelDepotData: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData,
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
    type Target = crate::System::Object;
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
        levelId: *mut crate::System::String,
        appId: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelId, appId))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        levelId: *mut crate::System::String,
        appId: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelId, appId))?;
        Ok(__cordl_ret)
    }
    pub fn get_appId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_appId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelDepotData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData = __cordl_object
            .invoke("get_levelDepotData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_levelId", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "SteamLevelProductCollectionModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductCollectionModel {
    __cordl_parent: crate::System::Object,
    pub _levelIdToProductData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
    >,
    pub _levelPackIdToProductData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
    >,
    pub _levelPackRedirectionData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
    >,
    pub _maxPossibleInstalledDepots: i32,
}
#[cfg(feature = "SteamLevelProductCollectionModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SteamLevelProductCollectionModel => ""
    ."SteamLevelProductCollectionModel"
);
#[cfg(feature = "SteamLevelProductCollectionModel")]
impl std::ops::Deref for SteamLevelProductCollectionModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel")]
impl std::ops::DerefMut for SteamLevelProductCollectionModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel")]
impl SteamLevelProductCollectionModel {
    #[cfg(feature = "SteamLevelProductCollectionModel+LevelProductData")]
    pub type LevelProductData = crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData;
    #[cfg(feature = "SteamLevelProductCollectionModel+LevelPackProductData")]
    pub type LevelPackProductData = crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData;
    #[cfg(feature = "SteamLevelProductCollectionModel+LevelDepotData")]
    pub type LevelDepotData = crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelDepotData;
    #[cfg(feature = "SteamLevelProductCollectionModel+LevelPackRedirectionData")]
    pub type LevelPackRedirectionData = crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData;
    pub fn GetLevelPackProductData(
        &mut self,
        levelPackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData = __cordl_object
            .invoke("GetLevelPackProductData", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelPackRedirectionData(
        &mut self,
        levelPackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData = __cordl_object
            .invoke("GetLevelPackRedirectionData", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelProductData(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData = __cordl_object
            .invoke("GetLevelProductData", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        levelProductsSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut SteamLevelProductPacksSO,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelProductsSOs))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        levelProductsSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut SteamLevelProductPacksSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelProductsSOs))?;
        Ok(__cordl_ret)
    }
    pub fn get_levelProductDataCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_levelProductDataCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelProductsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        > = __cordl_object.invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SteamLevelProductCollectionModel")]
impl quest_hook::libil2cpp::ObjectType for SteamLevelProductCollectionModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}