#[cfg(feature = "UnityEngine+UIElements+UxmlBoolAttributeDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlBoolAttributeDescription {
    __cordl_parent: crate::UnityEngine::UIElements::TypedUxmlAttributeDescription_1<
        bool,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlBoolAttributeDescription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UxmlBoolAttributeDescription => "UnityEngine.UIElements"
    ."UxmlBoolAttributeDescription"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlBoolAttributeDescription")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlBoolAttributeDescription {
    type Target = crate::UnityEngine::UIElements::TypedUxmlAttributeDescription_1<bool>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlBoolAttributeDescription")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UxmlBoolAttributeDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlBoolAttributeDescription")]
impl crate::UnityEngine::UIElements::UxmlBoolAttributeDescription {
    #[cfg(feature = "UnityEngine+UIElements+UxmlBoolAttributeDescription+__c")]
    pub type __c = crate::UnityEngine::UIElements::UxmlBoolAttributeDescription___c;
    pub fn GetValueFromBag(
        &mut self,
        bag: *mut crate::UnityEngine::UIElements::IUxmlAttributes,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetValueFromBag", (bag, cc))?;
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
}
#[cfg(feature = "UnityEngine+UIElements+UxmlBoolAttributeDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlBoolAttributeDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}