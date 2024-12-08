#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseFieldTraits_2<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<TValueType>,
    pub m_Value: TValueUxmlAttributeType,
    __cordl_phantom_TValueType: std::marker::PhantomData<TValueType>,
    __cordl_phantom_TValueUxmlAttributeType: std::marker::PhantomData<
        TValueUxmlAttributeType,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseFieldTraits_2 <
    TValueType, TValueUxmlAttributeType > => "UnityEngine.UIElements"."BaseFieldTraits`2"
    < TValueType, TValueUxmlAttributeType >
);
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    type Target = crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<TValueType>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    pub fn Init(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        bag: *mut crate::UnityEngine::UIElements::IUxmlAttributes,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValueUxmlAttributeType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValueUxmlAttributeType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
