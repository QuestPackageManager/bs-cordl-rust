#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactory_2")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlObjectFactory_2<
    TCreatedType: quest_hook::libil2cpp::Type,
    TTraits: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::BaseUxmlFactory_2<
        TCreatedType,
        TTraits,
    >,
    __cordl_phantom_TCreatedType: std::marker::PhantomData<TCreatedType>,
    __cordl_phantom_TTraits: std::marker::PhantomData<TTraits>,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactory_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UxmlObjectFactory_2 <
    TCreatedType, TTraits > => "UnityEngine.UIElements"."UxmlObjectFactory`2" <
    TCreatedType, TTraits >
);
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactory_2")]
impl<
    TCreatedType: quest_hook::libil2cpp::Type,
    TTraits: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::UIElements::UxmlObjectFactory_2<TCreatedType, TTraits> {
    type Target = crate::UnityEngine::UIElements::BaseUxmlFactory_2<
        TCreatedType,
        TTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactory_2")]
impl<
    TCreatedType: quest_hook::libil2cpp::Type,
    TTraits: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::UIElements::UxmlObjectFactory_2<TCreatedType, TTraits> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactory_2")]
impl<
    TCreatedType: quest_hook::libil2cpp::Type,
    TTraits: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UxmlObjectFactory_2<TCreatedType, TTraits> {
    pub fn CreateObject(
        &mut self,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<TCreatedType>
    where
        TCreatedType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TTraits: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TCreatedType = __cordl_object
            .invoke("CreateObject", (bag, cc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TCreatedType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TTraits: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        TTraits: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactory_2")]
impl<
    TCreatedType: quest_hook::libil2cpp::Type,
    TTraits: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlObjectFactory_2<TCreatedType, TTraits> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
