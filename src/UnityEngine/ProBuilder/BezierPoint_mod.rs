#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    pub fn CubicPosition(
        a: crate::UnityEngine::ProBuilder::BezierPoint,
        b: crate::UnityEngine::ProBuilder::BezierPoint,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CubicPosition", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
    }
    pub fn GetLookDirection(
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::BezierPoint>,
        index: i32,
        previous: i32,
        next: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLookDirection", (points, index, previous, next))?;
        Ok(__cordl_ret.into())
    }
    pub fn QuadraticPosition(
        a: crate::UnityEngine::ProBuilder::BezierPoint,
        b: crate::UnityEngine::ProBuilder::BezierPoint,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QuadraticPosition", (a, b, t))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
