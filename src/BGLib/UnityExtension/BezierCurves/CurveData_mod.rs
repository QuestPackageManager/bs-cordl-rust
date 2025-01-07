#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CurveData {
    pub startPoint: crate::UnityEngine::Vector3,
    pub endPoint: crate::UnityEngine::Vector3,
    pub startControlPoint: crate::UnityEngine::Vector3,
    pub endControlPoint: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::BezierCurves::CurveData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension.BezierCurves";
    const CLASS_NAME: &'static str = "CurveData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BGLib::UnityExtension::BezierCurves::CurveData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BGLib::UnityExtension::BezierCurves::CurveData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BGLib::UnityExtension::BezierCurves::CurveData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+CurveData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BGLib::UnityExtension::BezierCurves::CurveData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
