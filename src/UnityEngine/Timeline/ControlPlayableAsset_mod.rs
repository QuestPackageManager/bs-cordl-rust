#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct ControlPlayableAsset {
    __cordl_parent: crate::UnityEngine::Playables::PlayableAsset,
    pub sourceGameObject: crate::UnityEngine::ExposedReference_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub prefabGameObject: *mut crate::UnityEngine::GameObject,
    pub updateParticle: bool,
    pub particleRandomSeed: u32,
    pub updateDirector: bool,
    pub updateITimeControl: bool,
    pub searchHierarchy: bool,
    pub active: bool,
    pub postPlayback: crate::UnityEngine::Timeline::ActivationControlPlayable_PostPlaybackState,
    pub m_ControlDirectorAsset: *mut crate::UnityEngine::Playables::PlayableAsset,
    pub m_Duration: f64,
    pub m_SupportLoop: bool,
    pub _controllingDirectors_k__BackingField: bool,
    pub _controllingParticles_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::ControlPlayableAsset =>
    "UnityEngine.Timeline"."ControlPlayableAsset"
);
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
impl std::ops::Deref for crate::UnityEngine::Timeline::ControlPlayableAsset {
    type Target = crate::UnityEngine::Playables::PlayableAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::ControlPlayableAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
impl crate::UnityEngine::Timeline::ControlPlayableAsset {
    pub const k_MaxRandInt: i32 = 10000i32;
    #[cfg(
        feature = "UnityEngine+Timeline+ControlPlayableAsset+_GetControlableScripts_d__39"
    )]
    pub type _GetControlableScripts_d__39 = crate::UnityEngine::Timeline::ControlPlayableAsset__GetControlableScripts_d__39;
    pub fn CreateActivationPlayable(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        outplayables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Playables::Playable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateActivationPlayable", (root, graph, outplayables))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayable(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreatePlayable", (graph, go))?;
        Ok(__cordl_ret.into())
    }
    pub fn GatherProperties(
        &mut self,
        director: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
        driver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IPropertyCollector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherProperties", (director, driver))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponent<T>(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<T>,
        > = __cordl_object.invoke("GetComponent", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllableParticleSystems(
        &mut self,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ParticleSystem,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ParticleSystem,
            >,
        > = __cordl_object.invoke("GetControllableParticleSystems", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchHierarchyAndConnectDirector(
        &mut self,
        directors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Playables::PlayableDirector,
            >,
        >,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        outplayables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Playables::Playable,
            >,
        >,
        disableSelfReferences: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SearchHierarchyAndConnectDirector",
                (directors, graph, outplayables, disableSelfReferences),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchHierarchyAndConnectParticleSystem(
        &mut self,
        particleSystems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::ParticleSystem,
            >,
        >,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        outplayables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Playables::Playable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SearchHierarchyAndConnectParticleSystem",
                (particleSystems, graph, outplayables),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDurationAndLoopFlag(
        &mut self,
        directors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::Playables::PlayableDirector,
            >,
        >,
        particleSystems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ParticleSystem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDurationAndLoopFlag", (directors, particleSystems))?;
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
    pub fn get_clipCaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::ClipCaps> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::ClipCaps = __cordl_object
            .invoke("get_clipCaps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controllingDirectors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_controllingDirectors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controllingParticles(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_controllingParticles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_controllingDirectors(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controllingDirectors", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_controllingParticles(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controllingParticles", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::ControlPlayableAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
impl AsRef<crate::UnityEngine::Timeline::IPropertyPreview>
for crate::UnityEngine::Timeline::ControlPlayableAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::IPropertyPreview {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
impl AsMut<crate::UnityEngine::Timeline::IPropertyPreview>
for crate::UnityEngine::Timeline::ControlPlayableAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::IPropertyPreview {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
impl AsRef<crate::UnityEngine::Timeline::ITimelineClipAsset>
for crate::UnityEngine::Timeline::ControlPlayableAsset {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::ITimelineClipAsset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ControlPlayableAsset")]
impl AsMut<crate::UnityEngine::Timeline::ITimelineClipAsset>
for crate::UnityEngine::Timeline::ControlPlayableAsset {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::ITimelineClipAsset {
        unsafe { std::mem::transmute(self) }
    }
}
