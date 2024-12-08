#[cfg(feature = "IMissionNodesManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IMissionNodesManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMissionNodesManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IMissionNodesManager => ""."IMissionNodesManager"
);
#[cfg(feature = "IMissionNodesManager")]
impl std::ops::Deref for IMissionNodesManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMissionNodesManager")]
impl std::ops::DerefMut for IMissionNodesManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMissionNodesManager")]
impl IMissionNodesManager {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_allMissionNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            *mut IMissionNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            *mut IMissionNode,
        > = __cordl_object.invoke("get_allMissionNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_finalMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IMissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IMissionNode = __cordl_object
            .invoke("get_finalMissionNode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IMissionNodesManager")]
impl quest_hook::libil2cpp::ObjectType for IMissionNodesManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}