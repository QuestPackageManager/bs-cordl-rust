#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PointGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::PointGenerator =>
    "UnityEngine.ProBuilder.Poly2Tri"."PointGenerator"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointGenerator")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::PointGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointGenerator")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::PointGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointGenerator")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::PointGenerator {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UniformDistribution(
        n: i32,
        scale: f64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UniformDistribution", (n, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn UniformGrid(
        n: i32,
        scale: f64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UniformGrid", (n, scale))?;
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
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::PointGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
