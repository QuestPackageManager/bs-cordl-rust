#[cfg(feature = "cordl_class_UnityEngine+Timeline+PrefabControlPlayable")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabControlPlayable {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub m_Instance: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
}
#[cfg(feature = "cordl_class_UnityEngine+Timeline+PrefabControlPlayable")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Timeline::PrefabControlPlayable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "PrefabControlPlayable";
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
#[cfg(feature = "UnityEngine+Timeline+PrefabControlPlayable")]
impl std::ops::Deref for crate::UnityEngine::Timeline::PrefabControlPlayable {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+PrefabControlPlayable")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::PrefabControlPlayable {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+PrefabControlPlayable")]
impl crate::UnityEngine::Timeline::PrefabControlPlayable {
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        prefabGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::PrefabControlPlayable>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Playables::PlayableGraph,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    ), crate::UnityEngine::Playables::ScriptPlayable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Timeline::PrefabControlPlayable,
                        >,
                    >, 3usize>("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Create",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::PrefabControlPlayable>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (graph, prefabGameObject, parentTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        prefabGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>, 2usize>(
                        "Initialize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = unsafe {
            cordl_method_info.invoke_unchecked(self, (prefabGameObject, parentTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBehaviourPause(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Playables::Playable,
                        crate::UnityEngine::Playables::FrameData,
                    ), quest_hook::libil2cpp::Void, 2usize>("OnBehaviourPause")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnBehaviourPause",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (playable, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnBehaviourPlay(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Playables::Playable,
                        crate::UnityEngine::Playables::FrameData,
                    ), quest_hook::libil2cpp::Void, 2usize>("OnBehaviourPlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnBehaviourPlay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (playable, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnPlayableDestroy(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Playables::Playable),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnPlayableDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnPlayableDestroy", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (playable))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetHideFlagsRecursive(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetHideFlagsRecursive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetHideFlagsRecursive", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (gameObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_prefabInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        0usize,
                    >("get_prefabInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_prefabInstance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Timeline+PrefabControlPlayable")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::PrefabControlPlayable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
