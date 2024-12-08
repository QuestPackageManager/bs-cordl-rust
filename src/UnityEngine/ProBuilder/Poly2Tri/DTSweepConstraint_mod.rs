#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepConstraint")]
#[repr(C)]
#[derive(Debug)]
pub struct DTSweepConstraint {
    __cordl_parent: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationConstraint,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepConstraint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint =>
    "UnityEngine.ProBuilder.Poly2Tri"."DTSweepConstraint"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepConstraint")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint {
    type Target = crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationConstraint;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepConstraint")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepConstraint")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint {
    pub fn _ctor(
        &mut self,
        p1: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        p2: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p1, p2))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        p1: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        p2: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p1, p2))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweepConstraint")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
