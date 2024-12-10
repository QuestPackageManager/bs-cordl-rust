#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointOnEdgeException")]
#[repr(C)]
#[derive(Debug)]
pub struct PointOnEdgeException {
    __cordl_parent: crate::System::NotImplementedException,
    pub A: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    pub B: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    pub C: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointOnEdgeException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::PointOnEdgeException =>
    "UnityEngine.ProBuilder.Poly2Tri"."PointOnEdgeException"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointOnEdgeException")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::PointOnEdgeException {
    type Target = crate::System::NotImplementedException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointOnEdgeException")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::Poly2Tri::PointOnEdgeException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointOnEdgeException")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::PointOnEdgeException {
    pub fn New(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        a: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        c: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, a, b, c))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        a: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        c: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, a, b, c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointOnEdgeException")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::PointOnEdgeException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
