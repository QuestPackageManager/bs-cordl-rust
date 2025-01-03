#[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionSyncData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _status: i32,
    pub _timeLastUpdate: f32,
    pub _timeStartConnection: f32,
    pub _newNodeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _nodeTypeIndex: i32,
    pub _validForConnection: bool,
    pub _viewportHAPI: crate::HoudiniEngineUnity::HAPI_Viewport,
    pub _viewportLocal: crate::HoudiniEngineUnity::HAPI_Viewport,
    pub _viewportJustUpdated: bool,
    pub _syncInfo: crate::HoudiniEngineUnity::HAPI_SessionSyncInfo,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_SessionSyncData =>
    "HoudiniEngineUnity"."HEU_SessionSyncData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_SessionSyncData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_SessionSyncData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData")]
impl crate::HoudiniEngineUnity::HEU_SessionSyncData {
    #[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData+Status")]
    pub type Status = crate::HoudiniEngineUnity::HEU_SessionSyncData_Status;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_SyncStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_SessionSyncData_Status,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_SessionSyncData_Status = __cordl_object
            .invoke("get_SyncStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SyncStatus(
        &mut self,
        value: crate::HoudiniEngineUnity::HEU_SessionSyncData_Status,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SyncStatus", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_SessionSyncData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData+Status")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_SessionSyncData_Status {
    Connected = 4i32,
    Connecting = 2i32,
    Initializing = 3i32,
    Started = 1i32,
    Stopped = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionSyncData+Status")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_SessionSyncData_Status
    => "HoudiniEngineUnity"."HEU_SessionSyncData/Status"
);
