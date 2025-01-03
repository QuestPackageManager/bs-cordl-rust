#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+P2T")]
#[repr(C)]
#[derive(Debug)]
pub struct P2T {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+P2T")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Poly2Tri::P2T =>
    "UnityEngine.ProBuilder.Poly2Tri"."P2T"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+P2T")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::P2T {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+P2T")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::P2T {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+P2T")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::P2T {
    pub fn CreateContext(
        algorithm: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationAlgorithm,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateContext", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate_ConstrainedPointSet2(
        cps: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::ConstrainedPointSet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Triangulate", (cps))?;
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate_PointSet3(
        ps: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Poly2Tri::PointSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Triangulate", (ps))?;
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate_Polygon1(
        p: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Poly2Tri::Polygon>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Triangulate", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate_PolygonSet0(
        ps: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonSet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Triangulate", (ps))?;
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate_TriangulationAlgorithm_Triangulatable4(
        algorithm: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationAlgorithm,
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Triangulate", (algorithm, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate_TriangulationContext5(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Triangulate", (tcx))?;
        Ok(__cordl_ret.into())
    }
    pub fn Warmup() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Warmup", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+P2T")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::P2T {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
