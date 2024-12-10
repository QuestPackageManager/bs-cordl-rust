#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusChangeDirection")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementFocusChangeDirection {
    __cordl_parent: crate::UnityEngine::UIElements::FocusChangeDirection,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusChangeDirection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElementFocusChangeDirection =>
    "UnityEngine.UIElements"."VisualElementFocusChangeDirection"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusChangeDirection")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualElementFocusChangeDirection {
    type Target = crate::UnityEngine::UIElements::FocusChangeDirection;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusChangeDirection")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElementFocusChangeDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusChangeDirection")]
impl crate::UnityEngine::UIElements::VisualElementFocusChangeDirection {
    pub fn New(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusChangeDirection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementFocusChangeDirection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
