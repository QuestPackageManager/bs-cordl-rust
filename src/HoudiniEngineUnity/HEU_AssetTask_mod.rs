#[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AssetTask {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_Task,
    pub _buildType: crate::HoudiniEngineUnity::HEU_AssetTask_BuildType,
    pub _asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    pub _assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _position: crate::UnityEngine::Vector3,
    pub _buildResult: bool,
    pub _forceSessionID: i64,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_AssetTask =>
    "HoudiniEngineUnity"."HEU_AssetTask"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_AssetTask {
    type Target = crate::HoudiniEngineUnity::HEU_Task;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_AssetTask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask")]
impl crate::HoudiniEngineUnity::HEU_AssetTask {
    #[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask+BuildType")]
    pub type BuildType = crate::HoudiniEngineUnity::HEU_AssetTask_BuildType;
    pub fn CompleteTask(
        &mut self,
        result: crate::HoudiniEngineUnity::HEU_Task_TaskResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteTask", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn CookCompletedCallback_HEU_CookedEventData1(
        &mut self,
        cookedEventData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_CookedEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CookCompletedCallback", (cookedEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn CookCompletedCallback_HEU_HoudiniAsset__cordl_bool_List_1_0(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        bSuccess: bool,
        outputs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::GameObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CookCompletedCallback", (asset, bSuccess, outputs))?;
        Ok(__cordl_ret.into())
    }
    pub fn CookCompletedCallback_HEU_ReloadEventData2(
        &mut self,
        reloadEventData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ReloadEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CookCompletedCallback", (reloadEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoTask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTaskSession(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = __cordl_object.invoke("GetTaskSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn KillTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KillTask", ())?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_AssetTask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask+BuildType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_AssetTask_BuildType {
    COOK = 2i32,
    LOAD = 1i32,
    NONE = 0i32,
    RELOAD = 3i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetTask+BuildType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_AssetTask_BuildType =>
    "HoudiniEngineUnity"."HEU_AssetTask/BuildType"
);
