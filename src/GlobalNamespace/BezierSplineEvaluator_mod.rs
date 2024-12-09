#[cfg(feature = "BezierSplineEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct BezierSplineEvaluator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _segments: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::BezierCurve,
    >,
    pub _currentSegmentIndex: i32,
}
#[cfg(feature = "BezierSplineEvaluator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BezierSplineEvaluator => ""
    ."BezierSplineEvaluator"
);
#[cfg(feature = "BezierSplineEvaluator")]
impl std::ops::Deref for crate::GlobalNamespace::BezierSplineEvaluator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BezierSplineEvaluator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BezierSplineEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BezierSplineEvaluator")]
impl crate::GlobalNamespace::BezierSplineEvaluator {
    pub const kSlightAboveOne: f32 = 1.0005f32;
    pub const kSlightBelowZero: f32 = -0.0005f32;
    #[cfg(feature = "BezierSplineEvaluator+CubicSolveResult")]
    pub type CubicSolveResult = crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult;
    pub fn Evaluate(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("Evaluate", (t))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateFirstDerivation(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("EvaluateFirstDerivation", (t))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluatePosition(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("EvaluatePosition", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateSecondDerivation(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("EvaluateSecondDerivation", (t))?;
        Ok(__cordl_ret)
    }
    pub fn GetTForSegment(
        &mut self,
        segmentIndex: i32,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetTForSegment", (segmentIndex, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn GetTimeValuesForSegment(
        &mut self,
        segmentIndex: i32,
        t0Value: quest_hook::libil2cpp::ByRefMut<f32>,
        t1Value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTimeValuesForSegment", (segmentIndex, t0Value, t1Value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        spline: *mut crate::GlobalNamespace::BezierSpline,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (spline))?;
        Ok(__cordl_object)
    }
    pub fn OffsetSegmentAndGetT(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OffsetSegmentAndGetT", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn OffsetStartIndexToDistance(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OffsetStartIndexToDistance", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        spline: *mut crate::GlobalNamespace::BezierSpline,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (spline))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BezierSplineEvaluator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BezierSplineEvaluator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BezierSplineEvaluator+CubicSolveResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BezierSplineEvaluator_CubicSolveResult {
    pub numberOfSolutions: i32,
    pub solution1: f32,
    pub solution2: f32,
    pub solution3: f32,
}
#[cfg(feature = "BezierSplineEvaluator+CubicSolveResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult => ""
    ."BezierSplineEvaluator/CubicSolveResult"
);
#[cfg(feature = "BezierSplineEvaluator+CubicSolveResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BezierSplineEvaluator+CubicSolveResult")]
impl crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult {
    pub fn _ctor_f32_0(
        &mut self,
        solution1: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (solution1),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_1(
        &mut self,
        solution1: f32,
        solution2: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (solution1, solution2),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_f32_2(
        &mut self,
        solution1: f32,
        solution2: f32,
        solution3: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (solution1, solution2, solution3),
        )?;
        Ok(__cordl_ret)
    }
}
