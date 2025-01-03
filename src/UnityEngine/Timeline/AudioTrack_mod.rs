#[cfg(feature = "UnityEngine+Timeline+AudioTrack")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioTrack {
    __cordl_parent: crate::UnityEngine::Timeline::TrackAsset,
    pub m_TrackProperties: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Timeline::AudioMixerProperties,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+AudioTrack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::AudioTrack =>
    "UnityEngine.Timeline"."AudioTrack"
);
#[cfg(feature = "UnityEngine+Timeline+AudioTrack")]
impl std::ops::Deref for crate::UnityEngine::Timeline::AudioTrack {
    type Target = crate::UnityEngine::Timeline::TrackAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AudioTrack")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::AudioTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+AudioTrack")]
impl crate::UnityEngine::Timeline::AudioTrack {
    pub fn CompileClips(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        timelineClips: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::Timeline::TimelineClip,
            >,
        >,
        tree: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IntervalTree_1<
                *mut crate::UnityEngine::Timeline::RuntimeElement,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CompileClips", (graph, go, timelineClips, tree))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClip(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineClip,
        > = __cordl_object.invoke("CreateClip", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+Timeline+AudioTrack")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::AudioTrack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
