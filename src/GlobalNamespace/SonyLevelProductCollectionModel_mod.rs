#[cfg(feature = "SonyLevelProductCollectionModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductCollectionModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _levelIdToProductData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
    >,
    pub _levelPackIdToProductData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData,
    >,
    pub _levelPackRedirectionData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData,
    >,
}
#[cfg(feature = "SonyLevelProductCollectionModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SonyLevelProductCollectionModel
    => ""."SonyLevelProductCollectionModel"
);
#[cfg(feature = "SonyLevelProductCollectionModel")]
impl std::ops::Deref for crate::GlobalNamespace::SonyLevelProductCollectionModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyLevelProductCollectionModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel")]
impl crate::GlobalNamespace::SonyLevelProductCollectionModel {
    #[cfg(feature = "SonyLevelProductCollectionModel+AdditionalPackProductData")]
    pub type AdditionalPackProductData = crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData;
    #[cfg(feature = "SonyLevelProductCollectionModel+AdditionalProductData")]
    pub type AdditionalProductData = crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData;
    #[cfg(feature = "SonyLevelProductCollectionModel+LevelPackProductData")]
    pub type LevelPackProductData = crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData;
    #[cfg(feature = "SonyLevelProductCollectionModel+LevelPackRedirectionData")]
    pub type LevelPackRedirectionData = crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData;
    #[cfg(feature = "SonyLevelProductCollectionModel+LevelProductData")]
    pub type LevelProductData = crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData;
    #[cfg(feature = "SonyLevelProductCollectionModel+ProductData")]
    pub type ProductData = crate::GlobalNamespace::SonyLevelProductCollectionModel_ProductData;
    pub fn GetLevelPackProductData(
        &mut self,
        levelPackId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData = __cordl_object
            .invoke("GetLevelPackProductData", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelPackRedirectionData(
        &mut self,
        levelPackId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData = __cordl_object
            .invoke("GetLevelPackRedirectionData", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelProductData(
        &mut self,
        levelId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData = __cordl_object
            .invoke("GetLevelProductData", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sonyLevelProductPacksSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SonyLevelProductPackSO,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sonyLevelProductPacksSOs))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        sonyLevelProductPacksSOs: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SonyLevelProductPackSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sonyLevelProductPacksSOs))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductCollectionModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalPackProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductCollectionModel_AdditionalPackProductData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub checkAdditionalPackProductData: bool,
    pub noEnvironmentKeywordsProductLabel: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalPackProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData => ""
    ."SonyLevelProductCollectionModel/AdditionalPackProductData"
);
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalPackProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalPackProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalPackProductData")]
impl crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData {
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
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalPackProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductCollectionModel_AdditionalProductData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub checkAdditionalProductData: bool,
    pub noEnvironmentKeywordsProductData: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_ProductData,
}
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData => ""
    ."SonyLevelProductCollectionModel/AdditionalProductData"
);
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalProductData")]
impl crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData {
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
#[cfg(feature = "SonyLevelProductCollectionModel+AdditionalProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductCollectionModel_LevelPackProductData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _packId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _productLabel: *mut quest_hook::libil2cpp::Il2CppString,
    pub _packLevelPriceDiscountMul: f32,
    pub _levelProductsData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
    >,
    pub _packIndex: i32,
    pub _additionalPackProductData: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData,
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData => ""
    ."SonyLevelProductCollectionModel/LevelPackProductData"
);
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackProductData")]
impl crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData {
    pub fn CompareTo(
        &mut self,
        other: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        productLabel: *mut quest_hook::libil2cpp::Il2CppString,
        levelPackId: *mut quest_hook::libil2cpp::Il2CppString,
        packLevelPriceDiscountMul: f32,
        levelProductsData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
        >,
        packIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    productLabel,
                    levelPackId,
                    packLevelPriceDiscountMul,
                    levelProductsData,
                    packIndex,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        productLabel: *mut quest_hook::libil2cpp::Il2CppString,
        levelPackId: *mut quest_hook::libil2cpp::Il2CppString,
        packLevelPriceDiscountMul: f32,
        levelProductsData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
        >,
        packIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    productLabel,
                    levelPackId,
                    packLevelPriceDiscountMul,
                    levelProductsData,
                    packIndex,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_additionalPackProductData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalPackProductData = __cordl_object
            .invoke("get_additionalPackProductData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_levelPackId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelProductsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
        > = __cordl_object.invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_packIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packLevelPriceDiscountMul(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_packLevelPriceDiscountMul", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_productLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_productLabel", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackRedirectionData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductCollectionModel_LevelPackRedirectionData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _targetLevelPackId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _shouldOwnLevelPackId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _redirectedProductLabel: *mut quest_hook::libil2cpp::Il2CppString,
    pub _validUntilDate: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackRedirectionData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData => ""
    ."SonyLevelProductCollectionModel/LevelPackRedirectionData"
);
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackRedirectionData")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackRedirectionData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackRedirectionData")]
impl crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData {
    pub fn New(
        targetLevelPackId: *mut quest_hook::libil2cpp::Il2CppString,
        shouldOwnLevelPackId: *mut quest_hook::libil2cpp::Il2CppString,
        redirectedProductLabel: *mut quest_hook::libil2cpp::Il2CppString,
        validUntilDate: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    targetLevelPackId,
                    shouldOwnLevelPackId,
                    redirectedProductLabel,
                    validUntilDate,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        targetLevelPackId: *mut quest_hook::libil2cpp::Il2CppString,
        shouldOwnLevelPackId: *mut quest_hook::libil2cpp::Il2CppString,
        redirectedProductLabel: *mut quest_hook::libil2cpp::Il2CppString,
        validUntilDate: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    targetLevelPackId,
                    shouldOwnLevelPackId,
                    redirectedProductLabel,
                    validUntilDate,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_redirectedProductLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_redirectedProductLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shouldOwnLevelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_shouldOwnLevelPackId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetLevelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_targetLevelPackId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_validUntilDate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_validUntilDate", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelPackRedirectionData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackRedirectionData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductCollectionModel_LevelProductData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _levelId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _entitlementLabel: *mut quest_hook::libil2cpp::Il2CppString,
    pub _productLabel: *mut quest_hook::libil2cpp::Il2CppString,
    pub _sieeDcCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub _sieaAcCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub _additionalProductData: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData,
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData => ""
    ."SonyLevelProductCollectionModel/LevelProductData"
);
#[cfg(feature = "SonyLevelProductCollectionModel+LevelProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelProductData")]
impl crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData {
    pub fn New(
        entitlementLabel: *mut quest_hook::libil2cpp::Il2CppString,
        productLabel: *mut quest_hook::libil2cpp::Il2CppString,
        levelId: *mut quest_hook::libil2cpp::Il2CppString,
        sieeDcCode: *mut quest_hook::libil2cpp::Il2CppString,
        sieaAcCode: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (entitlementLabel, productLabel, levelId, sieeDcCode, sieaAcCode),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        entitlementLabel: *mut quest_hook::libil2cpp::Il2CppString,
        productLabel: *mut quest_hook::libil2cpp::Il2CppString,
        levelId: *mut quest_hook::libil2cpp::Il2CppString,
        sieeDcCode: *mut quest_hook::libil2cpp::Il2CppString,
        sieaAcCode: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (entitlementLabel, productLabel, levelId, sieeDcCode, sieaAcCode),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_additionalProductData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel_AdditionalProductData = __cordl_object
            .invoke("get_additionalProductData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_entitlementLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_entitlementLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_levelId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_productLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_productLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sieaAcCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_sieaAcCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sieeDcCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_sieeDcCode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+LevelProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+ProductData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductCollectionModel_ProductData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub entitlementLabel: *mut quest_hook::libil2cpp::Il2CppString,
    pub productLabel: *mut quest_hook::libil2cpp::Il2CppString,
    pub sieeDcCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub sieaAcCode: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "SonyLevelProductCollectionModel+ProductData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductCollectionModel_ProductData => ""
    ."SonyLevelProductCollectionModel/ProductData"
);
#[cfg(feature = "SonyLevelProductCollectionModel+ProductData")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductCollectionModel_ProductData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+ProductData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductCollectionModel_ProductData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductCollectionModel+ProductData")]
impl crate::GlobalNamespace::SonyLevelProductCollectionModel_ProductData {
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
#[cfg(feature = "SonyLevelProductCollectionModel+ProductData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductCollectionModel_ProductData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
