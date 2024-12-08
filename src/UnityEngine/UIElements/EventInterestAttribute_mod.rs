#[cfg(feature = "UnityEngine+UIElements+EventInterestAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EventInterestAttribute {
    __cordl_parent: crate::System::Attribute,
    pub eventTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    pub categoryFlags: crate::UnityEngine::UIElements::EventCategoryFlags,
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventInterestAttribute
    => "UnityEngine.UIElements"."EventInterestAttribute"
);
#[cfg(feature = "UnityEngine+UIElements+EventInterestAttribute")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventInterestAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EventInterestAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestAttribute")]
impl crate::UnityEngine::UIElements::EventInterestAttribute {
    pub fn New_EventInterestOptions1(
        interests: crate::UnityEngine::UIElements::EventInterestOptions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (interests))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray0(
        eventTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (eventTypes))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_EventInterestOptions1(
        &mut self,
        interests: crate::UnityEngine::UIElements::EventInterestOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (interests))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        eventTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (eventTypes))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventInterestAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}