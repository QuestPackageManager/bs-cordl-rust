#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationPoint")]
#[repr(C)]
#[derive(Debug)]
pub struct TriangulationPoint {
    __cordl_parent: crate::System::Object,
    pub _Edges_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
    >,
    pub X: f64,
    pub Y: f64,
    pub Index: i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationPoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint =>
    "UnityEngine.ProBuilder.Poly2Tri"."TriangulationPoint"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationPoint")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationPoint")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationPoint")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint {
    pub const INSERTED_INDEX: i32 = -1i32;
    pub const INVALID_INDEX: i32 = -2i32;
    pub fn get_HasEdges(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasEdges", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Yf(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Yf", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_Xf(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Xf", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Yf(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Yf", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Xf(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Xf", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Edges(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Edges", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Edges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        > = __cordl_object.invoke("get_Edges", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddEdge(
        &mut self,
        e: *mut crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEdge", (e))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        x: f64,
        y: f64,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (x, y, index))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationPoint")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
