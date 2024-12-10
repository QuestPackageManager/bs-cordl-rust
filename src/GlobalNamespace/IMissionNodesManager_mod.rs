#[cfg(feature = "IMissionNodesManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IMissionNodesManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMissionNodesManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IMissionNodesManager => ""
    ."IMissionNodesManager"
);
#[cfg(feature = "IMissionNodesManager")]
impl std::ops::Deref for crate::GlobalNamespace::IMissionNodesManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMissionNodesManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::IMissionNodesManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMissionNodesManager")]
impl crate::GlobalNamespace::IMissionNodesManager {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_allMissionNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<
                *mut crate::GlobalNamespace::IMissionNode,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<
                *mut crate::GlobalNamespace::IMissionNode,
            >,
        > = __cordl_object.invoke("get_allMissionNodes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_finalMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMissionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMissionNode,
        > = __cordl_object.invoke("get_finalMissionNode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IMissionNodesManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IMissionNodesManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
