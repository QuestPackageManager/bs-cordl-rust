#[cfg(feature = "MissionNode")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionNode {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionDataSO: *mut MissionDataSO,
    pub _letterPartName: *mut crate::System::String,
    pub _numberPartName: i32,
    pub _rectTransform: *mut crate::UnityEngine::RectTransform,
    pub _missionNodeVisualController: *mut MissionNodeVisualController,
    pub _childNodes: *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionNode>,
}
#[cfg(feature = "MissionNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionNode => ""."MissionNode"
);
#[cfg(feature = "MissionNode")]
impl std::ops::Deref for MissionNode {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNode")]
impl std::ops::DerefMut for MissionNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNode")]
impl MissionNode {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_position", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_formattedMissionNodeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_formattedMissionNodeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_letterPartName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_letterPartName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_radius(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_radius", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_childNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionNode> = __cordl_object
            .invoke("get_childNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_numberPartName(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberPartName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_missionId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missionNodeVisualController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionNodeVisualController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionNodeVisualController = __cordl_object
            .invoke("get_missionNodeVisualController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionDataSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionDataSO = __cordl_object
            .invoke("get_missionData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MissionNode")]
impl quest_hook::libil2cpp::ObjectType for MissionNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
