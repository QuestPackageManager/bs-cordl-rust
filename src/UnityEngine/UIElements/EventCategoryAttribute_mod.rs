#[cfg(feature = "UnityEngine+UIElements+EventCategoryAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EventCategoryAttribute {
    __cordl_parent: crate::System::Attribute,
    pub category: crate::UnityEngine::UIElements::EventCategory,
}
#[cfg(feature = "UnityEngine+UIElements+EventCategoryAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventCategoryAttribute
    => "UnityEngine.UIElements"."EventCategoryAttribute"
);
#[cfg(feature = "UnityEngine+UIElements+EventCategoryAttribute")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventCategoryAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCategoryAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EventCategoryAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCategoryAttribute")]
impl crate::UnityEngine::UIElements::EventCategoryAttribute {
    pub fn New(
        category: crate::UnityEngine::UIElements::EventCategory,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (category))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        category: crate::UnityEngine::UIElements::EventCategory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (category))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCategoryAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventCategoryAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}