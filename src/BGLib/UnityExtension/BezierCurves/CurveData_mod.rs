#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CurveData {
    pub startPoint: crate::UnityEngine::Vector3,
    pub endPoint: crate::UnityEngine::Vector3,
    pub startControlPoint: crate::UnityEngine::Vector3,
    pub endControlPoint: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::BezierCurves::CurveData
    => "BGLib.UnityExtension.BezierCurves"."CurveData"
);
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BGLib::UnityExtension::BezierCurves::CurveData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
impl crate::BGLib::UnityExtension::BezierCurves::CurveData {
    pub fn Evaluate(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Evaluate",
            (t),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RotatePointAroundPivot(
        point: crate::UnityEngine::Vector3,
        pivot: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotatePointAroundPivot", (point, pivot, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotatePointsAroundPivot(
        &mut self,
        pivot: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        crate::BGLib::UnityExtension::BezierCurves::CurveData,
    > {
        let __cordl_ret: crate::BGLib::UnityExtension::BezierCurves::CurveData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RotatePointsAroundPivot",
            (pivot, rot),
        )?;
        Ok(__cordl_ret.into())
    }
}
