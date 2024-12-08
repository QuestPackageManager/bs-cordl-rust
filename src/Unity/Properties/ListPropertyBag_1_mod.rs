#[cfg(feature = "Unity+Properties+ListPropertyBag_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ListPropertyBag_1<TElement: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Unity::Properties::IndexedCollectionPropertyBag_2<
        *mut crate::System::Collections::Generic::List_1<TElement>,
        TElement,
    >,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "Unity+Properties+ListPropertyBag_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::ListPropertyBag_1 < TElement
    > => "Unity.Properties"."ListPropertyBag`1" < TElement >
);
#[cfg(feature = "Unity+Properties+ListPropertyBag_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Properties::ListPropertyBag_1<TElement> {
    type Target = crate::Unity::Properties::IndexedCollectionPropertyBag_2<
        *mut crate::System::Collections::Generic::List_1<TElement>,
        TElement,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ListPropertyBag_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Properties::ListPropertyBag_1<TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ListPropertyBag_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::ListPropertyBag_1<TElement> {
    pub fn get_InstantiationKind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Properties::InstantiationKind>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Properties::InstantiationKind = __cordl_object
            .invoke("get_InstantiationKind", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateWithCount(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<TElement>,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<TElement> = __cordl_object
            .invoke("InstantiateWithCount", (count))?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<TElement>,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<TElement> = __cordl_object
            .invoke("Instantiate", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
#[cfg(feature = "Unity+Properties+ListPropertyBag_1")]
impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::ListPropertyBag_1<TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
