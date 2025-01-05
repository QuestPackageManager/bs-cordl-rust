#[cfg(feature = "SceneTransitionFadingExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneTransitionFadingExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "SceneTransitionFadingExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SceneTransitionFadingExtensions
    => ""."SceneTransitionFadingExtensions"
);
#[cfg(feature = "SceneTransitionFadingExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::SceneTransitionFadingExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SceneTransitionFadingExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::SceneTransitionFadingExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SceneTransitionFadingExtensions")]
impl crate::GlobalNamespace::SceneTransitionFadingExtensions {
    pub fn ShouldFadeOnSceneTransition(
        sceneTransitionType: crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldFadeOnSceneTransition", (sceneTransitionType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SceneTransitionFadingExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SceneTransitionFadingExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
