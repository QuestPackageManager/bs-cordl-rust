#[cfg(feature = "UnityEngine+UIElements+UxmlFactory_1")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlFactory_1<TCreatedType: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        TCreatedType,
        *mut crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
    >,
    __cordl_phantom_TCreatedType: std::marker::PhantomData<TCreatedType>,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlFactory_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UxmlFactory_1 <
    TCreatedType > => "UnityEngine.UIElements"."UxmlFactory`1" < TCreatedType >
);
#[cfg(feature = "UnityEngine+UIElements+UxmlFactory_1")]
impl<TCreatedType: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::UxmlFactory_1<TCreatedType> {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        TCreatedType,
        *mut crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlFactory_1")]
impl<TCreatedType: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::UxmlFactory_1<TCreatedType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlFactory_1")]
impl<
    TCreatedType: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UxmlFactory_1<TCreatedType> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TCreatedType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TCreatedType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlFactory_1")]
impl<TCreatedType: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlFactory_1<TCreatedType> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
