#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonSet")]
#[repr(C)]
#[derive(Debug)]
pub struct PolygonSet {
    __cordl_parent: crate::System::Object,
    pub _polygons: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Poly2Tri::PolygonSet =>
    "UnityEngine.ProBuilder.Poly2Tri"."PolygonSet"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonSet")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::PolygonSet {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonSet")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::PolygonSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonSet")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::PolygonSet {
    pub fn Add(
        &mut self,
        p: *mut crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (p))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Polygon1(
        poly: *mut crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (poly))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Polygon1(
        &mut self,
        poly: *mut crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (poly))?;
        Ok(__cordl_ret)
    }
    pub fn get_Polygons(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
        > = __cordl_object.invoke("get_Polygons", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonSet")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::PolygonSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}