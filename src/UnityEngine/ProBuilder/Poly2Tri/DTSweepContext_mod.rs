#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepContext")]
#[repr(C)]
#[derive(Debug)]
pub struct DTSweepContext {
    __cordl_parent: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
    pub ALPHA: f32,
    pub Front: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFront,
    >,
    pub _Head_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    >,
    pub _Tail_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    >,
    pub Basin: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepBasin,
    >,
    pub EdgeEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepEdgeEvent,
    >,
    pub _comparator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepPointComparator,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext =>
    "UnityEngine.ProBuilder.Poly2Tri"."DTSweepContext"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepContext")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext {
    type Target = crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepContext")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepContext")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext {
    pub fn AddNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNode", (node))?;
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
    pub fn CreateAdvancingFront(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateAdvancingFront", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeTriangulation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeTriangulation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateNode(
        &mut self,
        point: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        > = __cordl_object.invoke("LocateNode", (point))?;
        Ok(__cordl_ret.into())
    }
    pub fn MapTriangleToNodes(
        &mut self,
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MapTriangleToNodes", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn MeshClean(
        &mut self,
        triangle: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MeshClean", (triangle))?;
        Ok(__cordl_ret.into())
    }
    pub fn MeshCleanReq(
        &mut self,
        triangle: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MeshCleanReq", (triangle))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NewConstraint(
        &mut self,
        a: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationConstraint,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationConstraint,
        > = __cordl_object.invoke("NewConstraint", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareTriangulation(
        &mut self,
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareTriangulation", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromList(
        &mut self,
        triangle: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFromList", (triangle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNode", (node))?;
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
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationAlgorithm,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationAlgorithm = __cordl_object
            .invoke("get_Algorithm", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Head(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        > = __cordl_object.invoke("get_Head", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDebugEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDebugEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Tail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        > = __cordl_object.invoke("get_Tail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Head(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Head", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsDebugEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDebugEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Tail(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Tail", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
