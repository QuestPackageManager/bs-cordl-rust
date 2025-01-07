#[cfg(feature = "CubicBezierHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CubicBezierHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "CubicBezierHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::CubicBezierHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CubicBezierHelper";
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
#[cfg(feature = "CubicBezierHelper")]
impl std::ops::Deref for crate::GlobalNamespace::CubicBezierHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CubicBezierHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::CubicBezierHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CubicBezierHelper")]
impl crate::GlobalNamespace::CubicBezierHelper {
    pub fn EstimateCurveLength(
        p0: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p3: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EstimateCurveLength", (p0, p1, p2, p3))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateCurve(
        a1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        c1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        c2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        a2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvaluateCurve", (a1, c1, c2, a2, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateCurveDerivative(
        a1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        c1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        c2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        a2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvaluateCurveDerivative", (a1, c1, c2, a2, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateCurveSecondDerivative(
        a1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        c1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        c2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        a2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvaluateCurveSecondDerivative", (a1, c1, c2, a2, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normal(
        a1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        c1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        c2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        a2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normal", (a1, c1, c2, a2, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn SplitCurve(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SplitCurve", (points, t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CubicBezierHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CubicBezierHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
