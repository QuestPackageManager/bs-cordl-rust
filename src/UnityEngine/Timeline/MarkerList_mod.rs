#[cfg(feature = "UnityEngine+Timeline+MarkerList")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MarkerList {
    pub m_Objects: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ScriptableObject,
    >,
    pub m_Cache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Timeline::IMarker,
    >,
    pub m_CacheDirty: bool,
    pub m_HasNotifications: bool,
}
#[cfg(feature = "UnityEngine+Timeline+MarkerList")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::MarkerList =>
    "UnityEngine.Timeline"."MarkerList"
);
#[cfg(feature = "UnityEngine+Timeline+MarkerList")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Timeline::MarkerList {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MarkerList")]
impl crate::UnityEngine::Timeline::MarkerList {
    pub fn Add(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BuildCache",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMarker(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        _cordl_time: f64,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IMarker,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateMarker",
            (_cordl_type, _cordl_time, owner),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::IMarker,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::Timeline::IMarker,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetMarkers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawMarkerList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ScriptableObject,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ScriptableObject,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetRawMarkerList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasNotifications(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasNotifications",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_IMarker0(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_ScriptableObject_TimelineAsset_PlayableAsset1(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        timelineAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineAsset,
        >,
        thingToDirty: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (item, timelineAsset, thingToDirty),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (capacity),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IMarker,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_markers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::Timeline::IMarker,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::Timeline::IMarker,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_markers", ())?;
        Ok(__cordl_ret.into())
    }
}
