#[cfg(feature = "UnityEngine+UIElements+MouseOverEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseOverEvent {
    __cordl_parent: crate::UnityEngine::UIElements::MouseEventBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MouseOverEvent>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MouseOverEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MouseOverEvent =>
    "UnityEngine.UIElements"."MouseOverEvent"
);
#[cfg(feature = "UnityEngine+UIElements+MouseOverEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MouseOverEvent {
    type Target = crate::UnityEngine::UIElements::MouseEventBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MouseOverEvent>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseOverEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MouseOverEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseOverEvent")]
impl crate::UnityEngine::UIElements::MouseOverEvent {
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
#[cfg(feature = "UnityEngine+UIElements+MouseOverEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MouseOverEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
