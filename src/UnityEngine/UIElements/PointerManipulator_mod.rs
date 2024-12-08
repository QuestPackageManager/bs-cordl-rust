#[cfg(feature = "UnityEngine+UIElements+PointerManipulator")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerManipulator {
    __cordl_parent: crate::UnityEngine::UIElements::MouseManipulator,
    pub m_CurrentPointerId: i32,
}
#[cfg(feature = "UnityEngine+UIElements+PointerManipulator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerManipulator =>
    "UnityEngine.UIElements"."PointerManipulator"
);
#[cfg(feature = "UnityEngine+UIElements+PointerManipulator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerManipulator {
    type Target = crate::UnityEngine::UIElements::MouseManipulator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerManipulator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerManipulator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerManipulator")]
impl crate::UnityEngine::UIElements::PointerManipulator {
    pub fn CanStopManipulation(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::IPointerEvent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanStopManipulation", (e))?;
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
    pub fn CanStartManipulation(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::IPointerEvent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanStartManipulation", (e))?;
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
#[cfg(feature = "UnityEngine+UIElements+PointerManipulator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerManipulator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
