#[cfg(feature = "ILevelPackProductData_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ILevelPackProductData_1<TLevelProductData: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TLevelProductData: std::marker::PhantomData<TLevelProductData>,
}
#[cfg(feature = "ILevelPackProductData_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ILevelPackProductData_1 <
    TLevelProductData > => ""."ILevelPackProductData`1" < TLevelProductData >
);
#[cfg(feature = "ILevelPackProductData_1")]
impl<TLevelProductData: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::ILevelPackProductData_1<TLevelProductData> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ILevelPackProductData_1")]
impl<TLevelProductData: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::ILevelPackProductData_1<TLevelProductData> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ILevelPackProductData_1")]
impl<
    TLevelProductData: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::ILevelPackProductData_1<TLevelProductData> {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_levelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TLevelProductData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TLevelProductData>>
    where
        TLevelProductData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TLevelProductData> = __cordl_object
            .invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ILevelPackProductData_1")]
impl<TLevelProductData: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ILevelPackProductData_1<TLevelProductData> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
