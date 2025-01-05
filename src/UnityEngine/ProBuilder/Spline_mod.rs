#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
#[repr(C)]
#[derive(Debug)]
pub struct Spline {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Spline =>
    "UnityEngine.ProBuilder"."Spline"
);
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Spline {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Spline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl crate::UnityEngine::ProBuilder::Spline {
    pub fn Extrude__cordl_bool_ByRefMut_Gc2(
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
        radius: f32,
        radiusRows: i32,
        closeLoop: bool,
        smooth: bool,
        target: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
        pointRotations: quest_hook::libil2cpp::Gc<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Extrude",
                (points, radius, radiusRows, closeLoop, smooth, target, pointRotations),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Extrude_i32__cordl_bool0(
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::BezierPoint>,
        radius: f32,
        columns: i32,
        rows: i32,
        closeLoop: bool,
        smooth: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extrude", (points, radius, columns, rows, closeLoop, smooth))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extrude_i32__cordl_bool_ByRefMut1(
        bezierPoints: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::BezierPoint,
        >,
        radius: f32,
        columns: i32,
        rows: i32,
        closeLoop: bool,
        smooth: bool,
        target: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Extrude",
                (bezierPoints, radius, columns, rows, closeLoop, smooth, target),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControlPoints(
        bezierPoints: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::BezierPoint,
        >,
        subdivisionsPerSegment: i32,
        closeLoop: bool,
        rotations: quest_hook::libil2cpp::Gc<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetControlPoints",
                (bezierPoints, subdivisionsPerSegment, closeLoop, rotations),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRingRotation(
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
        i: i32,
        closeLoop: bool,
        secant: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRingRotation", (points, i, closeLoop, secant))?;
        Ok(__cordl_ret.into())
    }
    pub fn VertexRing(
        orientation: crate::UnityEngine::Quaternion,
        offset: crate::UnityEngine::Vector3,
        radius: f32,
        segments: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VertexRing", (orientation, offset, radius, segments))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Spline {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
