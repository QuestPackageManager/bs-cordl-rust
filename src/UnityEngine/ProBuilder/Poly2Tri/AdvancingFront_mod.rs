#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFront")]
#[repr(C)]
#[derive(Debug)]
pub struct AdvancingFront {
    __cordl_parent: crate::System::Object,
    pub Head: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    pub Tail: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    pub Search: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFront")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::AdvancingFront =>
    "UnityEngine.ProBuilder.Poly2Tri"."AdvancingFront"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFront")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFront {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFront")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFront {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFront")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFront {
    pub fn RemoveNode(
        &mut self,
        node: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        head: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        tail: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (head, tail))?;
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
    pub fn FindSearchNode(
        &mut self,
        x: f64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode = __cordl_object
            .invoke("FindSearchNode", (x))?;
        Ok(__cordl_ret)
    }
    pub fn LocateNode_TriangulationPoint0(
        &mut self,
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode = __cordl_object
            .invoke("LocateNode", (point))?;
        Ok(__cordl_ret)
    }
    pub fn LocateNode_f64_1(
        &mut self,
        x: f64,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode = __cordl_object
            .invoke("LocateNode", (x))?;
        Ok(__cordl_ret)
    }
    pub fn LocatePoint(
        &mut self,
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode = __cordl_object
            .invoke("LocatePoint", (point))?;
        Ok(__cordl_ret)
    }
    pub fn AddNode(
        &mut self,
        node: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        head: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        tail: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (head, tail))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFront")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFront {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
