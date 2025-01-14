#[cfg(feature = "UnityEngine+Audio+AudioPlayableGraphExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioPlayableGraphExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableGraphExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Audio::AudioPlayableGraphExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Audio";
    const CLASS_NAME: &'static str = "AudioPlayableGraphExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableGraphExtensions")]
impl std::ops::Deref for crate::UnityEngine::Audio::AudioPlayableGraphExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableGraphExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Audio::AudioPlayableGraphExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableGraphExtensions")]
impl crate::UnityEngine::Audio::AudioPlayableGraphExtensions {
    pub fn InternalCreateAudioOutput(
        graph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableGraph,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableGraph,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Playables::PlayableOutputHandle,
                    >,
                ),
                bool,
                3usize,
            >("InternalCreateAudioOutput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalCreateAudioOutput", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (graph, name, handle))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioPlayableGraphExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Audio::AudioPlayableGraphExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
