#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointSet")]
#[repr(C)]
#[derive(Debug)]
pub struct PointSet {
    __cordl_parent: crate::System::Object,
    pub _Points_k__BackingField: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
    >,
    pub _Triangles_k__BackingField: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Poly2Tri::PointSet =>
    "UnityEngine.ProBuilder.Poly2Tri"."PointSet"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointSet")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::PointSet {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointSet")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::PointSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointSet")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::PointSet {
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
    pub fn set_Triangles(
        &mut self,
        value: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Triangles", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Points(
        &mut self,
        value: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Points", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        points: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (points))?;
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
    pub fn New(
        points: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (points))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+PointSet")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::PointSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
