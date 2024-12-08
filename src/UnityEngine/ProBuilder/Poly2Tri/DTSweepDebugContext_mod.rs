#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepDebugContext")]
#[repr(C)]
#[derive(Debug)]
pub struct DTSweepDebugContext {
    __cordl_parent: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext,
    pub _primaryTriangle: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    pub _secondaryTriangle: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    pub _activePoint: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    pub _activeNode: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    pub _activeConstraint: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepDebugContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::DTSweepDebugContext =>
    "UnityEngine.ProBuilder.Poly2Tri"."DTSweepDebugContext"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepDebugContext")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepDebugContext {
    type Target = crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepDebugContext")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepDebugContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepDebugContext")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepDebugContext {
    pub fn get_PrimaryTriangle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle = __cordl_object
            .invoke("get_PrimaryTriangle", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ActiveNode(
        &mut self,
        value: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ActiveNode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SecondaryTriangle(
        &mut self,
        value: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SecondaryTriangle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ActivePoint(
        &mut self,
        value: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ActivePoint", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ActiveNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode = __cordl_object
            .invoke("get_ActiveNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ActivePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint = __cordl_object
            .invoke("get_ActivePoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ActiveConstraint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint = __cordl_object
            .invoke("get_ActiveConstraint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDebugContext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDebugContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_PrimaryTriangle(
        &mut self,
        value: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrimaryTriangle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ActiveConstraint(
        &mut self,
        value: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ActiveConstraint", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        tcx: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tcx))?;
        Ok(__cordl_ret)
    }
    pub fn get_SecondaryTriangle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle = __cordl_object
            .invoke("get_SecondaryTriangle", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        tcx: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tcx))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepDebugContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepDebugContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
