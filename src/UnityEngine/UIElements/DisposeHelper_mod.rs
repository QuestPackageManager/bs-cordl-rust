#[cfg(feature = "UnityEngine+UIElements+DisposeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct DisposeHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+DisposeHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DisposeHelper =>
    "UnityEngine.UIElements"."DisposeHelper"
);
#[cfg(feature = "UnityEngine+UIElements+DisposeHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DisposeHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DisposeHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DisposeHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DisposeHelper")]
impl crate::UnityEngine::UIElements::DisposeHelper {}
#[cfg(feature = "UnityEngine+UIElements+DisposeHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DisposeHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
