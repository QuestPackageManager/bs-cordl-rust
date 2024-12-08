#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BackgroundPropertyHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BackgroundPropertyHelper => "UnityEngine.UIElements"
    ."BackgroundPropertyHelper"
);
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl crate::UnityEngine::UIElements::BackgroundPropertyHelper {}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
