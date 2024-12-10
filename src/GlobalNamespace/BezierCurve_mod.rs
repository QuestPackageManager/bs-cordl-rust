#[cfg(feature = "BezierCurve")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BezierCurve {
    pub p0: crate::UnityEngine::Vector3,
    pub p1: crate::UnityEngine::Vector3,
    pub p2: crate::UnityEngine::Vector3,
    pub p3: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BezierCurve")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BezierCurve => ""."BezierCurve"
);
#[cfg(feature = "BezierCurve")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::BezierCurve {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BezierCurve")]
impl crate::GlobalNamespace::BezierCurve {
    pub fn _ctor(
        &mut self,
        p0: crate::UnityEngine::Vector3,
        p1: crate::UnityEngine::Vector3,
        p2: crate::UnityEngine::Vector3,
        p3: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (p0, p1, p2, p3),
        )?;
        Ok(__cordl_ret.into())
    }
}
