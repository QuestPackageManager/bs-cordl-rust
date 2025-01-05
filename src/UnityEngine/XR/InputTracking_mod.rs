#[cfg(feature = "UnityEngine+XR+InputTracking")]
#[repr(C)]
#[derive(Debug)]
pub struct InputTracking {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+XR+InputTracking")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::InputTracking =>
    "UnityEngine.XR"."InputTracking"
);
#[cfg(feature = "UnityEngine+XR+InputTracking")]
impl std::ops::Deref for crate::UnityEngine::XR::InputTracking {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+InputTracking")]
impl std::ops::DerefMut for crate::UnityEngine::XR::InputTracking {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+InputTracking")]
impl crate::UnityEngine::XR::InputTracking {
    #[cfg(feature = "UnityEngine+XR+InputTracking+TrackingStateEventType")]
    pub type TrackingStateEventType = crate::UnityEngine::XR::InputTracking_TrackingStateEventType;
    pub fn GetDeviceIdAtXRNode(
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDeviceIdAtXRNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeStates(
        nodeStates: quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRNodeState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeStates", (nodeStates))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeStates_Internal(
        nodeStates: quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRNodeState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeStates_Internal", (nodeStates))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeTrackingEvent(
        eventType: crate::UnityEngine::XR::InputTracking_TrackingStateEventType,
        nodeType: crate::UnityEngine::XR::XRNode,
        uniqueID: i64,
        tracked: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeTrackingEvent", (eventType, nodeType, uniqueID, tracked))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_nodeAdded(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRNodeState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_nodeAdded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_nodeRemoved(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRNodeState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_nodeRemoved", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_nodeAdded(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRNodeState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_nodeAdded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_nodeRemoved(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRNodeState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_nodeRemoved", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+InputTracking")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::InputTracking {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+XR+InputTracking+TrackingStateEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputTracking_TrackingStateEventType {
    #[default]
    NodeAdded = 0i32,
    NodeRemoved = 1i32,
    TrackingAcquired = 2i32,
    TrackingLost = 3i32,
}
#[cfg(feature = "UnityEngine+XR+InputTracking+TrackingStateEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::InputTracking_TrackingStateEventType => "UnityEngine.XR"
    ."InputTracking/TrackingStateEventType"
);
