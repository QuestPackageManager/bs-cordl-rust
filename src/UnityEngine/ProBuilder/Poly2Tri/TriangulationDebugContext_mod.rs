#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationDebugContext")]
#[repr(C)]
#[derive(Debug)]
pub struct TriangulationDebugContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tcx: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationDebugContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext =>
    "UnityEngine.ProBuilder.Poly2Tri"."TriangulationDebugContext"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationDebugContext")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationDebugContext")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationDebugContext")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext {
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
    pub fn New(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tcx))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tcx))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationDebugContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationDebugContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
