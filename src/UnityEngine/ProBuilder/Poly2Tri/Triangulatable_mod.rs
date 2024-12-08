#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Triangulatable")]
#[repr(C)]
#[derive(Debug)]
pub struct Triangulatable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Triangulatable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::Triangulatable =>
    "UnityEngine.ProBuilder.Poly2Tri"."Triangulatable"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Triangulatable")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Triangulatable")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Triangulatable")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable {
    pub fn Prepare(
        &mut self,
        tcx: *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Prepare", (tcx))?;
        Ok(__cordl_ret)
    }
    pub fn get_Points(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        > = __cordl_object.invoke("get_Points", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Triangles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        > = __cordl_object.invoke("get_Triangles", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddTriangle(
        &mut self,
        t: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTriangle", (t))?;
        Ok(__cordl_ret)
    }
    pub fn AddTriangles(
        &mut self,
        list: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTriangles", (list))?;
        Ok(__cordl_ret)
    }
    pub fn ClearTriangles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearTriangles", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TriangulationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationMode = __cordl_object
            .invoke("get_TriangulationMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Triangulatable")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
