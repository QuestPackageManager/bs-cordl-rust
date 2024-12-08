#[cfg(feature = "UnityEngine+Animations+AnimationPlayableExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationPlayableExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Animations::AnimationPlayableExtensions => "UnityEngine.Animations"
    ."AnimationPlayableExtensions"
);
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableExtensions")]
impl std::ops::Deref for crate::UnityEngine::Animations::AnimationPlayableExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Animations::AnimationPlayableExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableExtensions")]
impl crate::UnityEngine::Animations::AnimationPlayableExtensions {}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Animations::AnimationPlayableExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
