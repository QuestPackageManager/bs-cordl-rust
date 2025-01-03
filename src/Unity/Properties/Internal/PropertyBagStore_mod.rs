#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyBagStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::Internal::PropertyBagStore =>
    "Unity.Properties.Internal"."PropertyBagStore"
);
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
impl std::ops::Deref for crate::Unity::Properties::Internal::PropertyBagStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::PropertyBagStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
impl crate::Unity::Properties::Internal::PropertyBagStore {
    #[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
    pub type TypedStore_1<TContainer: quest_hook::libil2cpp::Type> = crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<
        TContainer,
    >;
    pub fn AddPropertyBag<TContainer>(
        propertyBag: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TContainer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddPropertyBag", (propertyBag))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyBag_0<TContainer>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag_1<TContainer>>,
    >
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TContainer>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyBag_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyBag", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::PropertyBagStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PropertyBagStore_TypedStore_1<TContainer: quest_hook::libil2cpp::Type> {
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::PropertyBagStore_TypedStore_1 < TContainer > =>
    "Unity.Properties.Internal"."PropertyBagStore/TypedStore`1<TContainer>" < TContainer
    >
);
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {}
