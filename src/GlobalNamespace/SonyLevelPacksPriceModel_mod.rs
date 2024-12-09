#[cfg(feature = "SonyLevelPacksPriceModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelPacksPriceModel {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _pricePerLevel: f32,
    pub _pricePairBundlePairs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair,
    >,
}
#[cfg(feature = "SonyLevelPacksPriceModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SonyLevelPacksPriceModel => ""
    ."SonyLevelPacksPriceModel"
);
#[cfg(feature = "SonyLevelPacksPriceModel")]
impl std::ops::Deref for crate::GlobalNamespace::SonyLevelPacksPriceModel {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelPacksPriceModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyLevelPacksPriceModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelPacksPriceModel")]
impl crate::GlobalNamespace::SonyLevelPacksPriceModel {
    #[cfg(feature = "SonyLevelPacksPriceModel+PricePerBundlePair")]
    pub type PricePerBundlePair = crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair;
    pub fn GetPricePerBundlePair(
        &mut self,
        levelsCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair = __cordl_object
            .invoke("GetPricePerBundlePair", (levelsCount))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_pricePairBundlePairs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair,
        > = __cordl_object.invoke("get_pricePairBundlePairs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pricePerLevel(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pricePerLevel", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyLevelPacksPriceModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelPacksPriceModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelPacksPriceModel+PricePerBundlePair")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelPacksPriceModel_PricePerBundlePair {
    __cordl_parent: crate::System::Object,
    pub _pricePerBundle: f32,
    pub _levelsCount: i32,
}
#[cfg(feature = "SonyLevelPacksPriceModel+PricePerBundlePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair => ""
    ."SonyLevelPacksPriceModel/PricePerBundlePair"
);
#[cfg(feature = "SonyLevelPacksPriceModel+PricePerBundlePair")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelPacksPriceModel+PricePerBundlePair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelPacksPriceModel+PricePerBundlePair")]
impl crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair {
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
    pub fn get_levelsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_levelsCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pricePerBundle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pricePerBundle", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyLevelPacksPriceModel+PricePerBundlePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelPacksPriceModel_PricePerBundlePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
