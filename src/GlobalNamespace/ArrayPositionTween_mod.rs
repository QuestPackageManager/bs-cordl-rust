#[cfg(feature = "ArrayPositionTween")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayPositionTween {
    __cordl_parent: crate::UnityEngine::Playables::PlayableAsset,
    pub arrayReference: crate::UnityEngine::ExposedReference_1<
        *mut crate::GlobalNamespace::TimelineArrayReference,
    >,
    pub elementDelay: f32,
    pub _cordl_template: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CustomTweenBehaviour,
    >,
}
#[cfg(feature = "ArrayPositionTween")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ArrayPositionTween => ""
    ."ArrayPositionTween"
);
#[cfg(feature = "ArrayPositionTween")]
impl std::ops::Deref for crate::GlobalNamespace::ArrayPositionTween {
    type Target = crate::UnityEngine::Playables::PlayableAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ArrayPositionTween")]
impl std::ops::DerefMut for crate::GlobalNamespace::ArrayPositionTween {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ArrayPositionTween")]
impl crate::GlobalNamespace::ArrayPositionTween {
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
}
#[cfg(feature = "ArrayPositionTween")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ArrayPositionTween {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ArrayPositionTween")]
impl AsRef<crate::UnityEngine::Timeline::ITimelineClipAsset>
for crate::GlobalNamespace::ArrayPositionTween {
    fn as_ref(&self) -> &crate::UnityEngine::Timeline::ITimelineClipAsset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ArrayPositionTween")]
impl AsMut<crate::UnityEngine::Timeline::ITimelineClipAsset>
for crate::GlobalNamespace::ArrayPositionTween {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Timeline::ITimelineClipAsset {
        unsafe { std::mem::transmute(self) }
    }
}
