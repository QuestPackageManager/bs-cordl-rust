#[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductCollectionModel_AdditionalSkus {
    __cordl_parent: crate::System::Object,
    pub checkAdditionalSkus: bool,
    pub noEnvironmentKeywordsSku: *mut crate::System::String,
}
#[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus => ""
    ."OculusLevelProductCollectionModel/AdditionalSkus"
);
#[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
impl std::ops::Deref
for crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
impl crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus {
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
#[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductCollectionModel_LevelPackProductData {
    __cordl_parent: crate::System::Object,
    pub _levelPackId: *mut crate::System::String,
    pub _sku: *mut crate::System::String,
    pub _additionalSkus: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
    pub _levelProductsData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
    >,
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData => ""
    ."OculusLevelProductCollectionModel/LevelPackProductData"
);
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
impl crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData {
    pub fn get_additionalSkus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus = __cordl_object
            .invoke("get_additionalSkus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sku(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_sku", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        levelPackId: *mut crate::System::String,
        sku: *mut crate::System::String,
        levelProductsData: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelPackId, sku, levelProductsData))?;
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
            *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        > = __cordl_object.invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        levelPackId: *mut crate::System::String,
        sku: *mut crate::System::String,
        levelProductsData: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelPackId, sku, levelProductsData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductCollectionModel_LevelPackRedirectionData {
    __cordl_parent: crate::System::Object,
    pub _targetLevelPackId: *mut crate::System::String,
    pub _shouldOwnLevelPackId: *mut crate::System::String,
    pub _redirectedSku: *mut crate::System::String,
    pub _validUntilDate: *mut crate::System::String,
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData => ""
    ."OculusLevelProductCollectionModel/LevelPackRedirectionData"
);
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
impl std::ops::Deref
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
impl crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData {
    pub fn get_redirectedSku(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_redirectedSku", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductCollectionModel_LevelProductData {
    __cordl_parent: crate::System::Object,
    pub _levelId: *mut crate::System::String,
    pub _sku: *mut crate::System::String,
    pub _additionalSkus: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData => ""
    ."OculusLevelProductCollectionModel/LevelProductData"
);
#[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
impl crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData {
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
    pub fn get_sku(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_sku", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        levelId: *mut crate::System::String,
        sku: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelId, sku))?;
        Ok(__cordl_ret)
    }
    pub fn get_additionalSkus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus = __cordl_object
            .invoke("get_additionalSkus", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        levelId: *mut crate::System::String,
        sku: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelId, sku))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductCollectionModel {
    __cordl_parent: crate::System::Object,
    pub _levelIdToProductData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
    >,
    pub _levelPackIdToProductData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
    >,
    pub _levelPackRedirectionData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData,
    >,
    pub _assetFileToSku: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusLevelProductCollectionModel => ""
    ."OculusLevelProductCollectionModel"
);
#[cfg(feature = "OculusLevelProductCollectionModel")]
impl std::ops::Deref for OculusLevelProductCollectionModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
impl std::ops::DerefMut for OculusLevelProductCollectionModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
impl OculusLevelProductCollectionModel {
    #[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
    pub type AdditionalSkus = crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus;
    #[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
    pub type LevelProductData = crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData;
    #[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
    pub type LevelPackProductData = crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData;
    #[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
    pub type LevelPackRedirectionData = crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData;
    pub fn _ctor(
        &mut self,
        levelProductPacksSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut OculusLevelProductPacksSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelProductPacksSOs))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelPackProductData(
        &mut self,
        levelPackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData = __cordl_object
            .invoke("GetLevelPackProductData", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelSku(
        &mut self,
        assetFile: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetLevelSku", (assetFile))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelPackRedirectionData(
        &mut self,
        levelPackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData = __cordl_object
            .invoke("GetLevelPackRedirectionData", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelProductData(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData = __cordl_object
            .invoke("GetLevelProductData", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn InsertAdditionalSkuIfValid(
        &mut self,
        levelId: *mut crate::System::String,
        additionalSku: *mut crate::System::String,
        additionalSkuType: BeatmapLevelDataVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InsertAdditionalSkuIfValid",
                (levelId, additionalSku, additionalSkuType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        levelProductPacksSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut OculusLevelProductPacksSO,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelProductPacksSOs))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
impl quest_hook::libil2cpp::ObjectType for OculusLevelProductCollectionModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
