#[cfg(feature = "UnityEngine+Timeline+MarkerTrack")]
#[repr(C)]
#[derive(Debug)]
pub struct MarkerTrack {
    __cordl_parent: crate::UnityEngine::Timeline::TrackAsset,
}
#[cfg(feature = "UnityEngine+Timeline+MarkerTrack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::MarkerTrack =>
    "UnityEngine.Timeline"."MarkerTrack"
);
#[cfg(feature = "UnityEngine+Timeline+MarkerTrack")]
impl std::ops::Deref for crate::UnityEngine::Timeline::MarkerTrack {
    type Target = crate::UnityEngine::Timeline::TrackAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MarkerTrack")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::MarkerTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MarkerTrack")]
impl crate::UnityEngine::Timeline::MarkerTrack {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_outputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::Playables::PlayableBinding,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::Playables::PlayableBinding,
        > = __cordl_object.invoke("get_outputs", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Timeline+MarkerTrack")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::MarkerTrack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
