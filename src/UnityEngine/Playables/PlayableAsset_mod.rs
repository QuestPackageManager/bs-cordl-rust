#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayableAsset {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
}
#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Playables::PlayableAsset {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Playables";
    const CLASS_NAME: &'static str = "PlayableAsset";
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
#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
impl std::ops::Deref for crate::UnityEngine::Playables::PlayableAsset {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
impl std::ops::DerefMut for crate::UnityEngine::Playables::PlayableAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
impl crate::UnityEngine::Playables::PlayableAsset {
    pub fn CreatePlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Playables::PlayableGraph,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                ),
                crate::UnityEngine::Playables::Playable,
                2usize,
            >("CreatePlayable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreatePlayable", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Playables::Playable = unsafe {
            method.invoke_unchecked(self, (graph, owner))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreatePlayable(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::PlayableAsset>,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Playables::PlayableAsset,
                    >,
                    crate::UnityEngine::Playables::PlayableGraph,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Internal_CreatePlayable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_CreatePlayable", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, graph, go, ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetPlayableAssetDuration(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::PlayableAsset>,
        ptrToDouble: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Playables::PlayableAsset,
                    >,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Internal_GetPlayableAssetDuration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_GetPlayableAssetDuration", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, ptrToDouble))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f64, 0usize>("get_duration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_duration", 0usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_outputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::Playables::PlayableBinding,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        crate::UnityEngine::Playables::PlayableBinding,
                    >,
                >,
                0usize,
            >("get_outputs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_outputs", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::Playables::PlayableBinding,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Playables::PlayableAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
impl AsRef<crate::UnityEngine::Playables::IPlayableAsset>
for crate::UnityEngine::Playables::PlayableAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayableAsset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
impl AsMut<crate::UnityEngine::Playables::IPlayableAsset>
for crate::UnityEngine::Playables::PlayableAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayableAsset {
        unsafe { std::mem::transmute(self) }
    }
}
