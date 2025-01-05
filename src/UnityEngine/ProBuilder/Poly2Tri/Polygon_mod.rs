#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Polygon")]
#[repr(C)]
#[derive(Debug)]
pub struct Polygon {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _points: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
            >,
        >,
    >,
    pub _steinerPoints: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
            >,
        >,
    >,
    pub _holes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Poly2Tri::Polygon>,
        >,
    >,
    pub _triangles: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
            >,
        >,
    >,
    pub _last: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Polygon")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Poly2Tri::Polygon =>
    "UnityEngine.ProBuilder.Poly2Tri"."Polygon"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Polygon")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::Polygon {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Polygon")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::Polygon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Polygon")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::Polygon {
    pub fn AddHole(
        &mut self,
        poly: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddHole", (poly))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPoint(
        &mut self,
        p: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPoint", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPoints(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPoints", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSteinerPoint(
        &mut self,
        point: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSteinerPoint", (point))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSteinerPoints(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSteinerPoints", (points))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTriangle(
        &mut self,
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTriangle", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTriangles(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTriangles", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearSteinerPoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSteinerPoints", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearTriangles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearTriangles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertPointAfter(
        &mut self,
        point: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
        >,
        newPoint: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertPointAfter", (point, newPoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IEnumerable_1_1(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (points))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList_1_0(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (points))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray2(
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (points))?;
        Ok(__cordl_object.into())
    }
    pub fn Prepare(
        &mut self,
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Prepare", (tcx))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePoint(
        &mut self,
        p: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePoint", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_1(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (points))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList_1_0(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (points))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::Poly2Tri::PolygonPoint,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (points))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Holes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::Polygon,
                >,
            >,
        > = __cordl_object.invoke("get_Holes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Points(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                >,
            >,
        > = __cordl_object.invoke("get_Points", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Triangles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                >,
            >,
        > = __cordl_object.invoke("get_Triangles", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Polygon")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::Polygon {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Polygon")]
impl AsRef<crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable>
for crate::UnityEngine::ProBuilder::Poly2Tri::Polygon {
    fn as_ref(&self) -> &crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+Polygon")]
impl AsMut<crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable>
for crate::UnityEngine::ProBuilder::Poly2Tri::Polygon {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ProBuilder::Poly2Tri::Triangulatable {
        unsafe { std::mem::transmute(self) }
    }
}
