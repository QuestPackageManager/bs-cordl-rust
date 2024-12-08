#[cfg(feature = "UnityEngine+UIElements+StyleSelector")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSelector {
    __cordl_parent: crate::System::Object,
    pub m_Parts: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::StyleSelectorPart,
    >,
    pub m_PreviousRelationship: crate::UnityEngine::UIElements::StyleSelectorRelationship,
    pub pseudoStateMask: i32,
    pub negatedPseudoStateMask: i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSelector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleSelector =>
    "UnityEngine.UIElements"."StyleSelector"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSelector")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSelector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSelector")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSelector")]
impl crate::UnityEngine::UIElements::StyleSelector {
    #[cfg(feature = "UnityEngine+UIElements+StyleSelector+__c")]
    pub type __c = crate::UnityEngine::UIElements::StyleSelector___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
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
    pub fn get_parts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleSelectorPart,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleSelectorPart,
        > = __cordl_object.invoke("get_parts", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_previousRelationship(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSelectorRelationship,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSelectorRelationship = __cordl_object
            .invoke("get_previousRelationship", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_parts(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleSelectorPart,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_parts", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_previousRelationship(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSelectorRelationship,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_previousRelationship", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSelector")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSelector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}