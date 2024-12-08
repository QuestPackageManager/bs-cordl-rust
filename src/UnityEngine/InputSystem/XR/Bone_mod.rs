#[cfg(feature = "UnityEngine+InputSystem+XR+Bone")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bone {
    pub m_ParentBoneIndex: u32,
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_Rotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Bone")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::Bone =>
    "UnityEngine.InputSystem.XR"."Bone"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+Bone")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Bone {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Bone")]
impl crate::UnityEngine::InputSystem::XR::Bone {
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_position",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_parentBoneIndex(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_parentBoneIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_rotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rotation",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_position",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_parentBoneIndex(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_parentBoneIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
