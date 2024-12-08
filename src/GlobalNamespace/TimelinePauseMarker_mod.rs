#[cfg(feature = "TimelinePauseMarker")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelinePauseMarker {
    __cordl_parent: crate::UnityEngine::Timeline::Marker,
    pub _id_k__BackingField: crate::UnityEngine::PropertyName,
}
#[cfg(feature = "TimelinePauseMarker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TimelinePauseMarker => ""
    ."TimelinePauseMarker"
);
#[cfg(feature = "TimelinePauseMarker")]
impl std::ops::Deref for crate::GlobalNamespace::TimelinePauseMarker {
    type Target = crate::UnityEngine::Timeline::Marker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelinePauseMarker")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelinePauseMarker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelinePauseMarker")]
impl crate::GlobalNamespace::TimelinePauseMarker {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PropertyName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::PropertyName = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TimelinePauseMarker")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TimelinePauseMarker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
