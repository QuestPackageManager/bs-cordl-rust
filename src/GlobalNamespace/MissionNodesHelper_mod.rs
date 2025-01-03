#[cfg(feature = "MissionNodesHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionNodesHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MissionNodesHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionNodesHelper => ""
    ."MissionNodesHelper"
);
#[cfg(feature = "MissionNodesHelper")]
impl std::ops::Deref for crate::GlobalNamespace::MissionNodesHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodesHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionNodesHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodesHelper")]
impl crate::GlobalNamespace::MissionNodesHelper {
    pub fn CycleDetection_MissionNode0(
        node: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CycleDetection", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn CycleDetection_i32_Dictionary_2_1(
        node: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        layer: i32,
        layers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::GlobalNamespace::MissionNode,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CycleDetection", (node, layer, layers))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalNodeIsFinal_HashSet_1_1(
        finalNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        node: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        visitedNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut crate::GlobalNamespace::MissionNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FinalNodeIsFinal", (finalNode, node, visitedNodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalNodeIsFinal_MissionNode_MissionNode0(
        finalNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        rootNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FinalNodeIsFinal", (finalNode, rootNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllNodesFromRoot(
        root: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut crate::GlobalNamespace::MissionNode,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut crate::GlobalNamespace::MissionNode,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllNodesFromRoot", (root))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn VisitAllTree(
        node: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        visitedNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut crate::GlobalNamespace::MissionNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VisitAllTree", (node, visitedNodes))?;
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
#[cfg(feature = "MissionNodesHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MissionNodesHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
