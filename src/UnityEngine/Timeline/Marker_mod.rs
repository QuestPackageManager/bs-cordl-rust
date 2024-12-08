#[cfg(feature = "UnityEngine+Timeline+Marker")]
#[repr(C)]
#[derive(Debug)]
pub struct Marker {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_Time: f64,
    pub _parent_k__BackingField: *mut crate::UnityEngine::Timeline::TrackAsset,
}
#[cfg(feature = "UnityEngine+Timeline+Marker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::Marker =>
    "UnityEngine.Timeline"."Marker"
);
#[cfg(feature = "UnityEngine+Timeline+Marker")]
impl std::ops::Deref for crate::UnityEngine::Timeline::Marker {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+Marker")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::Marker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+Marker")]
impl crate::UnityEngine::Timeline::Marker {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnInitialize(
        &mut self,
        aPent: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnInitialize", (aPent))?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_Timeline_IMarker_Initialize(
        &mut self,
        parentTrack: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.Timeline.IMarker.Initialize", (parentTrack))?;
        Ok(__cordl_ret)
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
    pub fn get_parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Timeline::TrackAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::TrackAsset = __cordl_object
            .invoke("get_parent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_parent(
        &mut self,
        value: *mut crate::UnityEngine::Timeline::TrackAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_parent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_time(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_time", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Timeline+Marker")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::Marker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
