#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayableAsset {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
}
#[cfg(feature = "UnityEngine+Playables+PlayableAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableAsset =>
    "UnityEngine.Playables"."PlayableAsset"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayable", (graph, owner))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_duration", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::Playables::PlayableBinding,
            >,
        > = __cordl_object.invoke("get_outputs", ())?;
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
