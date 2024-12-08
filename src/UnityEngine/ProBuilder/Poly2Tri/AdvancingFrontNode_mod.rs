#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFrontNode")]
#[repr(C)]
#[derive(Debug)]
pub struct AdvancingFrontNode {
    __cordl_parent: crate::System::Object,
    pub Next: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    pub Prev: *mut crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
    pub Value: f64,
    pub Point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    pub Triangle: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFrontNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode =>
    "UnityEngine.ProBuilder.Poly2Tri"."AdvancingFrontNode"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFrontNode")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFrontNode")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFrontNode")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode {
    pub fn New(
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (point))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        point: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (point))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasPrev(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasPrev", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+AdvancingFrontNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}