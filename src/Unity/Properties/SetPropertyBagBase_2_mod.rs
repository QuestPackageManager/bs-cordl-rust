#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
#[repr(C)]
#[derive(Debug)]
pub struct SetPropertyBagBase_2<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::PropertyBag_1<TSet>,
    pub m_Property: *mut crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<
        TSet,
        TElement,
    >,
    __cordl_phantom_TSet: std::marker::PhantomData<TSet>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::SetPropertyBagBase_2 < TSet,
    TElement > => "Unity.Properties"."SetPropertyBagBase`2" < TSet, TElement >
);
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement> {
    type Target = crate::Unity::Properties::PropertyBag_1<TSet>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement> {
    #[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
    pub type SetElementProperty = crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<
        TSet,
        TElement,
    >;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TSet: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSet: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
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
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct SetPropertyBagBase_2_SetElementProperty<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::Property_2<TSet, TElement>,
    pub m_Value: TElement,
    __cordl_phantom_TSet: std::marker::PhantomData<TSet>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::SetPropertyBagBase_2_SetElementProperty < TSet, TElement > =>
    "Unity.Properties"."SetPropertyBagBase`2/SetElementProperty" < TSet, TElement >
);
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement> {
    type Target = crate::Unity::Properties::Property_2<TSet, TElement>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement> {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TSet: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSet: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
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
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString>
    where
        TSet: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
