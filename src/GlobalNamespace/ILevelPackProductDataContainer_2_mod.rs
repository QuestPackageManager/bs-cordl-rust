#[cfg(feature = "ILevelPackProductDataContainer_2")]
#[repr(C)]
#[derive(Debug)]
pub struct ILevelPackProductDataContainer_2<
    TLevelPackProductData: quest_hook::libil2cpp::Type,
    TLevelProductData: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TLevelPackProductData: std::marker::PhantomData<
        TLevelPackProductData,
    >,
    __cordl_phantom_TLevelProductData: std::marker::PhantomData<TLevelProductData>,
}
#[cfg(feature = "ILevelPackProductDataContainer_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ILevelPackProductDataContainer_2 < TLevelPackProductData,
    TLevelProductData > => ""."ILevelPackProductDataContainer`2" < TLevelPackProductData,
    TLevelProductData >
);
#[cfg(feature = "ILevelPackProductDataContainer_2")]
impl<
    TLevelPackProductData: quest_hook::libil2cpp::Type,
    TLevelProductData: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::GlobalNamespace::ILevelPackProductDataContainer_2<
    TLevelPackProductData,
    TLevelProductData,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ILevelPackProductDataContainer_2")]
impl<
    TLevelPackProductData: quest_hook::libil2cpp::Type,
    TLevelProductData: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::GlobalNamespace::ILevelPackProductDataContainer_2<
    TLevelPackProductData,
    TLevelProductData,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ILevelPackProductDataContainer_2")]
impl<
    TLevelPackProductData: quest_hook::libil2cpp::Type,
    TLevelProductData: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::ILevelPackProductDataContainer_2<
    TLevelPackProductData,
    TLevelProductData,
> {
    pub fn SetLevelPackProductData(
        &mut self,
        newProductPack: TLevelPackProductData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TLevelPackProductData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TLevelProductData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLevelPackProductData", (newProductPack))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_levelPackProductData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TLevelPackProductData>
    where
        TLevelPackProductData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TLevelProductData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TLevelPackProductData = __cordl_object
            .invoke("get_levelPackProductData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ILevelPackProductDataContainer_2")]
impl<
    TLevelPackProductData: quest_hook::libil2cpp::Type,
    TLevelProductData: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ILevelPackProductDataContainer_2<
    TLevelPackProductData,
    TLevelProductData,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
