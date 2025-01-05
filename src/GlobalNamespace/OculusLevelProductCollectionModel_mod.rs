#[cfg(feature = "OculusLevelProductCollectionModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductCollectionModel {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _levelIdToProductData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    >,
    pub _levelPackIdToProductData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData,
        >,
    >,
    pub _levelPackRedirectionData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData,
        >,
    >,
    pub _assetFileToSku: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusLevelProductCollectionModel => ""
    ."OculusLevelProductCollectionModel"
);
#[cfg(feature = "OculusLevelProductCollectionModel")]
impl std::ops::Deref for crate::GlobalNamespace::OculusLevelProductCollectionModel {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusLevelProductCollectionModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
impl crate::GlobalNamespace::OculusLevelProductCollectionModel {
    #[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
    pub type AdditionalSkus = crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus;
    #[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
    pub type LevelPackProductData = crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData;
    #[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
    pub type LevelPackRedirectionData = crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData;
    #[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
    pub type LevelProductData = crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData;
    pub fn GetLevelPackProductData(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        > = __cordl_object.invoke("GetLevelPackProductData", (levelPackId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelPackRedirectionData(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackRedirectionData,
        > = __cordl_object.invoke("GetLevelPackRedirectionData", (levelPackId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelProductData(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        > = __cordl_object.invoke("GetLevelProductData", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelSku(
        &mut self,
        assetFile: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetLevelSku", (assetFile))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertAdditionalSkuIfValid(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        additionalSku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        additionalSkuType: crate::GlobalNamespace::BeatmapLevelDataVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InsertAdditionalSkuIfValid",
                (levelId, additionalSku, additionalSkuType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        levelProductPacksSOs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OculusLevelProductPacksSO>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelProductPacksSOs))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelProductPacksSOs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OculusLevelProductPacksSO>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelProductPacksSOs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusLevelProductCollectionModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+AdditionalSkus")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductCollectionModel_AdditionalSkus {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub checkAdditionalSkus: bool,
    pub noEnvironmentKeywordsSku: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _additionalSkus: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
    >,
    pub _levelProductsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
            >,
        >,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn New(
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelProductsData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelPackId, sku, levelProductsData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelProductsData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelPackId, sku, levelProductsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_additionalSkus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
        > = __cordl_object.invoke("get_additionalSkus", ())?;
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
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
            >,
        > = __cordl_object.invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sku(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_sku", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackProductData")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelPackProductData {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelPackRedirectionData")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLevelProductCollectionModel_LevelPackRedirectionData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _targetLevelPackId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _shouldOwnLevelPackId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _redirectedSku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _validUntilDate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn get_redirectedSku(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_redirectedSku", ())?;
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _additionalSkus: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
    >,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn get_additionalSkus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusLevelProductCollectionModel_AdditionalSkus,
        > = __cordl_object.invoke("get_additionalSkus", ())?;
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
    pub fn get_sku(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_sku", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILevelProductData>>
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILevelProductData> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusLevelProductCollectionModel+LevelProductData")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILevelProductData>>
for crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILevelProductData> {
        unsafe { std::mem::transmute(self) }
    }
}
