#[cfg(feature = "UnityEngine+Animations+AnimationPlayableBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationPlayableBinding {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Animations::AnimationPlayableBinding => "UnityEngine.Animations"
    ."AnimationPlayableBinding"
);
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableBinding")]
impl std::ops::Deref for crate::UnityEngine::Animations::AnimationPlayableBinding {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableBinding")]
impl std::ops::DerefMut for crate::UnityEngine::Animations::AnimationPlayableBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableBinding")]
impl crate::UnityEngine::Animations::AnimationPlayableBinding {}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Animations::AnimationPlayableBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}