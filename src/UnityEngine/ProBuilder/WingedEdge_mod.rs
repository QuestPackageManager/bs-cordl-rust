#[cfg(feature = "UnityEngine+ProBuilder+WingedEdge")]
#[repr(C)]
#[derive(Debug)]
pub struct WingedEdge {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _edge_k__BackingField: crate::UnityEngine::ProBuilder::EdgeLookup,
    pub _face_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::Face,
    >,
    pub _next_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::WingedEdge,
    >,
    pub _previous_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::WingedEdge,
    >,
    pub _opposite_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::WingedEdge,
    >,
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdge")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::WingedEdge {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "WingedEdge";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdge")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::WingedEdge {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdge")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::WingedEdge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdge")]
impl crate::UnityEngine::ProBuilder::WingedEdge {
    pub fn Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_WingedEdge0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAdjacentEdgeWithCommonIndex(
        &mut self,
        common: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::WingedEdge,
        > = __cordl_object.invoke("GetAdjacentEdgeWithCommonIndex", (common))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpokes(
        wings: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::WingedEdge,
                        >,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::WingedEdge,
                        >,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetSpokes", (wings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWingedEdges_IEnumerable_1__cordl_bool1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        oneWingPerFace: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWingedEdges", (mesh, faces, oneWingPerFace))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWingedEdges__cordl_bool0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        oneWingPerFace: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWingedEdges", (mesh, oneWingPerFace))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeQuad(
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeQuad", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SortCommonIndexesByAdjacency(
        wings: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        >,
        common: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortCommonIndexesByAdjacency", (wings, common))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortEdgesByAdjacency_Face0(
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortEdgesByAdjacency", (face))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortEdgesByAdjacency_List_1_1(
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortEdgesByAdjacency", (edges))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
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
    pub fn get_edge(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::EdgeLookup> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::EdgeLookup = __cordl_object
            .invoke("get_edge", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_face(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = __cordl_object.invoke("get_face", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::WingedEdge,
        > = __cordl_object.invoke("get_next", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_opposite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::WingedEdge,
        > = __cordl_object.invoke("get_opposite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previous(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::WingedEdge,
        > = __cordl_object.invoke("get_previous", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_edge(
        &mut self,
        value: crate::UnityEngine::ProBuilder::EdgeLookup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_edge", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_face(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_face", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_next(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_next", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_opposite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_opposite", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_previous(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_previous", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdge")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::WingedEdge {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdge")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    >,
> for crate::UnityEngine::ProBuilder::WingedEdge {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdge")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    >,
> for crate::UnityEngine::ProBuilder::WingedEdge {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
