#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlLongAttributeDescription {
    __cordl_parent: crate::UnityEngine::UIElements::TypedUxmlAttributeDescription_1<i64>,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UxmlLongAttributeDescription => "UnityEngine.UIElements"
    ."UxmlLongAttributeDescription"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    type Target = crate::UnityEngine::UIElements::TypedUxmlAttributeDescription_1<i64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
impl crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    #[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription+__c")]
    pub type __c = crate::UnityEngine::UIElements::UxmlLongAttributeDescription___c;
    pub fn ConvertValueToLong(
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultValue: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertValueToLong", (v, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueFromBag(
        &mut self,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetValueFromBag", (bag, cc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
