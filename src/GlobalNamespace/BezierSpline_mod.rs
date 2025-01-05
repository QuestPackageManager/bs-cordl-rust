#[cfg(feature = "BezierSpline")]
#[repr(C)]
#[derive(Debug)]
pub struct BezierSpline {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _segments: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierCurve>,
    pub _sourceDataPoints: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "BezierSpline")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BezierSpline => ""
    ."BezierSpline"
);
#[cfg(feature = "BezierSpline")]
impl std::ops::Deref for crate::GlobalNamespace::BezierSpline {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BezierSpline")]
impl std::ops::DerefMut for crate::GlobalNamespace::BezierSpline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BezierSpline")]
impl crate::GlobalNamespace::BezierSpline {
    #[cfg(feature = "BezierSpline+ComputeControlPointsResults")]
    pub type ComputeControlPointsResults = crate::GlobalNamespace::BezierSpline_ComputeControlPointsResults;
    pub fn AddArtificialStartAndFinishPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddArtificialStartAndFinishPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPoint(
        &mut self,
        distance: f32,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPoint", (distance, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeControlPoints_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeControlPoints", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeControlPoints_Gc1(
        &mut self,
        k: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BezierSpline_ComputeControlPointsResults,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BezierSpline_ComputeControlPointsResults = __cordl_object
            .invoke("ComputeControlPoints", (k))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SortSourceData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortSourceData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_segments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierCurve>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BezierCurve,
        > = __cordl_object.invoke("get_segments", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BezierSpline")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BezierSpline {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BezierSpline+ComputeControlPointsResults")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BezierSpline_ComputeControlPointsResults {
    pub p1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub p2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
}
#[cfg(feature = "BezierSpline+ComputeControlPointsResults")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BezierSpline_ComputeControlPointsResults => ""
    ."BezierSpline/ComputeControlPointsResults"
);
#[cfg(feature = "BezierSpline+ComputeControlPointsResults")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BezierSpline_ComputeControlPointsResults {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BezierSpline+ComputeControlPointsResults")]
impl crate::GlobalNamespace::BezierSpline_ComputeControlPointsResults {
    pub fn _ctor(
        &mut self,
        p1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        p2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (p1, p2),
        )?;
        Ok(__cordl_ret.into())
    }
}
