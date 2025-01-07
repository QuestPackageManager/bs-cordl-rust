#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BezierPoint {
    pub position: crate::UnityEngine::Vector3,
    pub tangentIn: crate::UnityEngine::Vector3,
    pub tangentOut: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::BezierPoint {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "BezierPoint";
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
#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ProBuilder::BezierPoint {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ProBuilder::BezierPoint {
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
#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ProBuilder::BezierPoint {
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
#[cfg(feature = "UnityEngine+ProBuilder+BezierPoint")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ProBuilder::BezierPoint {
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
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::BezierPoint,
            >,
        >,
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
