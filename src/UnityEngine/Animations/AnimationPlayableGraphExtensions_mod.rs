#[cfg(feature = "UnityEngine+Animations+AnimationPlayableGraphExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationPlayableGraphExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableGraphExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Animations::AnimationPlayableGraphExtensions =>
    "UnityEngine.Animations"."AnimationPlayableGraphExtensions"
);
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableGraphExtensions")]
impl std::ops::Deref
for crate::UnityEngine::Animations::AnimationPlayableGraphExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableGraphExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::Animations::AnimationPlayableGraphExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableGraphExtensions")]
impl crate::UnityEngine::Animations::AnimationPlayableGraphExtensions {}
#[cfg(feature = "UnityEngine+Animations+AnimationPlayableGraphExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Animations::AnimationPlayableGraphExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
