#[cfg(feature = "UnityEngine+XR+XRNodeState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XRNodeState {
    pub m_Type: crate::UnityEngine::XR::XRNode,
    pub m_AvailableFields: crate::UnityEngine::XR::AvailableTrackingData,
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_Rotation: crate::UnityEngine::Quaternion,
    pub m_Velocity: crate::UnityEngine::Vector3,
    pub m_AngularVelocity: crate::UnityEngine::Vector3,
    pub m_Acceleration: crate::UnityEngine::Vector3,
    pub m_AngularAcceleration: crate::UnityEngine::Vector3,
    pub m_Tracked: i32,
    pub m_UniqueID: u64,
}
#[cfg(feature = "UnityEngine+XR+XRNodeState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRNodeState => "UnityEngine.XR"
    ."XRNodeState"
);
#[cfg(feature = "UnityEngine+XR+XRNodeState")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::XR::XRNodeState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+XRNodeState")]
impl crate::UnityEngine::XR::XRNodeState {
    pub fn TryGetAcceleration(
        &mut self,
        acceleration: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetAcceleration",
            (acceleration),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetAngularAcceleration(
        &mut self,
        angularAcceleration: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetAngularAcceleration",
            (angularAcceleration),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetAngularVelocity(
        &mut self,
        angularVelocity: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetAngularVelocity",
            (angularVelocity),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPosition(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetPosition",
            (position),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetRotation(
        &mut self,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetRotation",
            (rotation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetVelocity(
        &mut self,
        velocity: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetVelocity",
            (velocity),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGet_Quaternion1(
        &mut self,
        inValue: crate::UnityEngine::Quaternion,
        availabilityFlag: crate::UnityEngine::XR::AvailableTrackingData,
        outValue: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGet",
            (inValue, availabilityFlag, outValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGet_Vector3_0(
        &mut self,
        inValue: crate::UnityEngine::Vector3,
        availabilityFlag: crate::UnityEngine::XR::AvailableTrackingData,
        outValue: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGet",
            (inValue, availabilityFlag, outValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::XRNode> {
        let __cordl_ret: crate::UnityEngine::XR::XRNode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_nodeType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_nodeType(
        &mut self,
        value: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_nodeType",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tracked(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_tracked",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uniqueID(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_uniqueID",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
