#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BezierPoint {
    pub position: crate::UnityEngine::Vector3,
    pub tangentIn: crate::UnityEngine::Vector3,
    pub tangentOut: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::BezierPoint =>
    "UnityEngine.ProBuilder"."BezierPoint"
);
#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::BezierPoint {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
impl crate::UnityEngine::ProBuilder::BezierPoint {
    pub fn EnforceTangentMode(
        &mut self,
        master: crate::UnityEngine::ProBuilder::BezierTangentDirection,
        mode: crate::UnityEngine::ProBuilder::BezierTangentMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EnforceTangentMode",
            (master, mode),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetPosition(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetPosition",
            (position),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetTangentIn(
        &mut self,
        tangent: crate::UnityEngine::Vector3,
        mode: crate::UnityEngine::ProBuilder::BezierTangentMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTangentIn",
            (tangent, mode),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetTangentOut(
        &mut self,
        tangent: crate::UnityEngine::Vector3,
        mode: crate::UnityEngine::ProBuilder::BezierTangentMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTangentOut",
            (tangent, mode),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        position: crate::UnityEngine::Vector3,
        tangentIn: crate::UnityEngine::Vector3,
        tangentOut: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (position, tangentIn, tangentOut, rotation),
        )?;
        Ok(__cordl_ret)
    }
}
