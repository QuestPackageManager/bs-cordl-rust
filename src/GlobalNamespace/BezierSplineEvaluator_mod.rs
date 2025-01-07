#[cfg(feature = "BezierSplineEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct BezierSplineEvaluator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _segments: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::GlobalNamespace::BezierCurve>,
    >,
    pub _currentSegmentIndex: i32,
}
#[cfg(feature = "BezierSplineEvaluator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BezierSplineEvaluator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BezierSplineEvaluator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn CubeRoot(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CubeRoot", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Evaluate(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("Evaluate", (t))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        spline: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierSpline>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (spline))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SolveCubic(
        a: f32,
        b: f32,
        c: f32,
        d: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult,
    > {
        let __cordl_ret: crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SolveCubic", (a, b, c, d))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        spline: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierSpline>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (spline))?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BezierSplineEvaluator_CubicSolveResult {
    pub numberOfSolutions: i32,
    pub solution1: f32,
    pub solution2: f32,
    pub solution3: f32,
}
#[cfg(feature = "BezierSplineEvaluator+CubicSolveResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CubicSolveResult";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BezierSplineEvaluator_CubicSolveResult {
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
