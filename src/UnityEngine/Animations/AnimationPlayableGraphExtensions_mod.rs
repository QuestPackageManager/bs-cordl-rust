#[cfg(feature = "UnityEngine+Animations+AnimationPlayableGraphExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationPlayableGraphExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::UnityEngine::Animations::AnimationPlayableGraphExtensions {
    pub fn InternalCreateAnimationOutput(
        graph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableGraph,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalCreateAnimationOutput", (graph, name, handle))?;
        Ok(__cordl_ret.into())
    }
}
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
