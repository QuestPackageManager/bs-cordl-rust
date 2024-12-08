#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DelaunayTriangle")]
#[repr(C)]
#[derive(Debug)]
pub struct DelaunayTriangle {
    __cordl_parent: crate::System::Object,
    pub Points: crate::UnityEngine::ProBuilder::Poly2Tri::FixedArray3_1<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    >,
    pub Neighbors: crate::UnityEngine::ProBuilder::Poly2Tri::FixedArray3_1<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    >,
    pub EdgeIsConstrained: crate::UnityEngine::ProBuilder::Poly2Tri::FixedBitArray3,
    pub EdgeIsDelaunay: crate::UnityEngine::ProBuilder::Poly2Tri::FixedBitArray3,
    pub _IsInterior_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DelaunayTriangle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle =>
    "UnityEngine.ProBuilder.Poly2Tri"."DelaunayTriangle"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DelaunayTriangle")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DelaunayTriangle")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DelaunayTriangle")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle {
    pub fn SetDelaunayEdgeCCW(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        ce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDelaunayEdgeCCW", (p, ce))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsInterior(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInterior", ())?;
        Ok(__cordl_ret)
    }
    pub fn IndexCWFrom(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexCWFrom", (p))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkNeighbor_TriangulationPoint_TriangulationPoint_DelaunayTriangle0(
        &mut self,
        p1: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        p2: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        t: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkNeighbor", (p1, p2, t))?;
        Ok(__cordl_ret)
    }
    pub fn MarkNeighbor_DelaunayTriangle1(
        &mut self,
        t: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkNeighbor", (t))?;
        Ok(__cordl_ret)
    }
    pub fn OppositePoint(
        &mut self,
        t: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint = __cordl_object
            .invoke("OppositePoint", (t, p))?;
        Ok(__cordl_ret)
    }
    pub fn MarkNeighborEdges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkNeighborEdges", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetConstrainedEdgeAcross(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetConstrainedEdgeAcross", (p))?;
        Ok(__cordl_ret)
    }
    pub fn RotateCW(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RotateCW", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetConstrainedEdgeCW(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetConstrainedEdgeCW", (p))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstrainedEdgeCCW(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        ce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConstrainedEdgeCCW", (p, ce))?;
        Ok(__cordl_ret)
    }
    pub fn SetDelaunayEdgeCW(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        ce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDelaunayEdgeCW", (p, ce))?;
        Ok(__cordl_ret)
    }
    pub fn EdgeIndex(
        &mut self,
        p1: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        p2: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EdgeIndex", (p1, p2))?;
        Ok(__cordl_ret)
    }
    pub fn Centroid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint = __cordl_object
            .invoke("Centroid", ())?;
        Ok(__cordl_ret)
    }
    pub fn Area(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("Area", ())?;
        Ok(__cordl_ret)
    }
    pub fn NeighborAcrossFrom(
        &mut self,
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle = __cordl_object
            .invoke("NeighborAcrossFrom", (point))?;
        Ok(__cordl_ret)
    }
    pub fn SetDelaunayEdgeAcross(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        ce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDelaunayEdgeAcross", (p, ce))?;
        Ok(__cordl_ret)
    }
    pub fn PointCWFrom(
        &mut self,
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint = __cordl_object
            .invoke("PointCWFrom", (point))?;
        Ok(__cordl_ret)
    }
    pub fn Legalize(
        &mut self,
        oPoint: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        nPoint: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Legalize", (oPoint, nPoint))?;
        Ok(__cordl_ret)
    }
    pub fn PointCCWFrom(
        &mut self,
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint = __cordl_object
            .invoke("PointCCWFrom", (point))?;
        Ok(__cordl_ret)
    }
    pub fn GetConstrainedEdgeCCW(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetConstrainedEdgeCCW", (p))?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (p))?;
        Ok(__cordl_ret)
    }
    pub fn MarkEdge_DelaunayTriangle0(
        &mut self,
        triangle: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkEdge", (triangle))?;
        Ok(__cordl_ret)
    }
    pub fn MarkEdge_List_1_1(
        &mut self,
        tList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkEdge", (tList))?;
        Ok(__cordl_ret)
    }
    pub fn GetDelaunayEdgeCW(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetDelaunayEdgeCW", (p))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        p1: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        p2: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        p3: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p1, p2, p3))?;
        Ok(__cordl_ret)
    }
    pub fn NeighborCWFrom(
        &mut self,
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle = __cordl_object
            .invoke("NeighborCWFrom", (point))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstrainedEdgeAcross(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        ce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConstrainedEdgeAcross", (p, ce))?;
        Ok(__cordl_ret)
    }
    pub fn IndexCCWFrom(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexCCWFrom", (p))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstrainedEdgeCW(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        ce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConstrainedEdgeCW", (p, ce))?;
        Ok(__cordl_ret)
    }
    pub fn NeighborCCWFrom(
        &mut self,
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle = __cordl_object
            .invoke("NeighborCCWFrom", (point))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsInterior(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsInterior", (value))?;
        Ok(__cordl_ret)
    }
    pub fn MarkConstrainedEdge_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkConstrainedEdge", (index))?;
        Ok(__cordl_ret)
    }
    pub fn MarkConstrainedEdge_DTSweepConstraint1(
        &mut self,
        edge: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkConstrainedEdge", (edge))?;
        Ok(__cordl_ret)
    }
    pub fn MarkConstrainedEdge_TriangulationPoint_TriangulationPoint2(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        q: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkConstrainedEdge", (p, q))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (p))?;
        Ok(__cordl_ret)
    }
    pub fn GetDelaunayEdgeCCW(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetDelaunayEdgeCCW", (p))?;
        Ok(__cordl_ret)
    }
    pub fn GetDelaunayEdgeAcross(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetDelaunayEdgeAcross", (p))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        p1: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        p2: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        p3: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p1, p2, p3))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DelaunayTriangle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
