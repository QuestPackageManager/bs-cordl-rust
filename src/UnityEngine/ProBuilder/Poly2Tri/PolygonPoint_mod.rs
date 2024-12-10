#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonPoint")]
#[repr(C)]
#[derive(Debug)]
pub struct PolygonPoint {
    __cordl_parent: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    pub _Next_k__BackingField: *mut crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
    pub _Previous_k__BackingField: *mut crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonPoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint
    => "UnityEngine.ProBuilder.Poly2Tri"."PolygonPoint"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonPoint")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint {
    type Target = crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonPoint")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonPoint")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint {
    pub fn New(
        x: f64,
        y: f64,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (x, y, index))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        x: f64,
        y: f64,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (x, y, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
        > = __cordl_object.invoke("get_Next", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Previous(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
        > = __cordl_object.invoke("get_Previous", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Next(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Next", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Previous(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Previous", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PolygonPoint")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
