#[cfg(feature = "cordl_class_SceneTransitionFadingExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneTransitionFadingExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_SceneTransitionFadingExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::SceneTransitionFadingExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SceneTransitionFadingExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "SceneTransitionFadingExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::SceneTransitionFadingExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SceneTransitionFadingExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::SceneTransitionFadingExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SceneTransitionFadingExtensions")]
impl crate::GlobalNamespace::SceneTransitionFadingExtensions {
    pub fn ShouldFadeOnSceneTransition(
        sceneTransitionType: crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::GameScenesManager_SceneTransitionType),
                        bool,
                        1usize,
                    >("ShouldFadeOnSceneTransition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShouldFadeOnSceneTransition", 1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (sceneTransitionType))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_SceneTransitionFadingExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SceneTransitionFadingExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
