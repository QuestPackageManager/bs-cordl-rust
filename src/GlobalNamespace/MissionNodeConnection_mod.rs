#[cfg(feature = "MissionNodeConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionNodeConnection {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _separator: f32,
    pub _width: f32,
    pub _rectTransform: *mut crate::UnityEngine::RectTransform,
    pub _image: *mut crate::UnityEngine::UI::Image,
    pub _parentMissionNode: *mut crate::GlobalNamespace::MissionNodeVisualController,
    pub _childMissionNode: *mut crate::GlobalNamespace::MissionNodeVisualController,
    pub _animator: *mut crate::UnityEngine::Animator,
    pub _parentMissionNodePosition: crate::UnityEngine::Vector2,
    pub _childMissionNodePosition: crate::UnityEngine::Vector2,
    pub _isActive: bool,
}
#[cfg(feature = "MissionNodeConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionNodeConnection => ""
    ."MissionNodeConnection"
);
#[cfg(feature = "MissionNodeConnection")]
impl std::ops::Deref for crate::GlobalNamespace::MissionNodeConnection {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodeConnection")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionNodeConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodeConnection")]
impl crate::GlobalNamespace::MissionNodeConnection {
    pub fn MissionConnectionEnabledDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MissionConnectionEnabledDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetActive(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActive", (animated))?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        parentMissionNode: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNodeVisualController,
        >,
        childMissionNode: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNodeVisualController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (parentMissionNode, childMissionNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateConnectionRectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateConnectionRectTransform", ())?;
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
    pub fn get_childMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNodeVisualController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNodeVisualController,
        > = __cordl_object.invoke("get_childMissionNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parentMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNodeVisualController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNodeVisualController,
        > = __cordl_object.invoke("get_parentMissionNode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionNodeConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionNodeConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
