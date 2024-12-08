#[cfg(feature = "OVRSceneRoom")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneRoom {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _Floor_k__BackingField: *mut OVRScenePlane,
    pub _Ceiling_k__BackingField: *mut OVRScenePlane,
    pub _Walls_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut OVRScenePlane,
    >,
    pub _walls: *mut crate::System::Collections::Generic::List_1<*mut OVRScenePlane>,
    pub _orderedRoomGuids: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::Guid,
        i32,
    >,
    pub _wallOrderComparer: *mut crate::System::Comparison_1<*mut OVRScenePlane>,
    pub _sceneAnchor: *mut OVRSceneAnchor,
    pub _sceneManager: *mut OVRSceneManager,
    pub _uuidToQuery: *mut crate::System::Collections::Generic::HashSet_1<
        crate::System::Guid,
    >,
    pub _roomAnchors: *mut crate::System::Collections::Generic::List_1<OVRAnchor>,
    pub _taskCount: i32,
    pub _onFetchAnchorsCompleted: *mut crate::System::Action_1<bool>,
    pub _onAnchorLocalizationCompleted: *mut crate::System::Action_2<bool, OVRAnchor>,
}
#[cfg(feature = "OVRSceneRoom")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRSceneRoom => ""."OVRSceneRoom"
);
#[cfg(feature = "OVRSceneRoom")]
impl std::ops::Deref for OVRSceneRoom {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl std::ops::DerefMut for OVRSceneRoom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl OVRSceneRoom {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUuidsToQuery(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUuidsToQuery", ())?;
        Ok(__cordl_ret)
    }
    pub fn IOVRSceneComponent_Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IOVRSceneComponent.Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadRoom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRoom", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnFetchAnchorsCompleted(
        &mut self,
        success: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFetchAnchorsCompleted", (success))?;
        Ok(__cordl_ret)
    }
    pub fn OnLocalizationCompleted(
        &mut self,
        success: bool,
        anchor: OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLocalizationCompleted", (success, anchor))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateRoomInformation(
        &mut self,
        plane: *mut OVRScenePlane,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRoomInformation", (plane))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__24_0(
        &mut self,
        planeA: *mut OVRScenePlane,
        planeB: *mut OVRScenePlane,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("<Awake>b__24_0", (planeA, planeB))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_g__TryGetUuid_24_1(
        &mut self,
        plane: *mut OVRScenePlane,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<Awake>g__TryGetUuid|24_1", (plane, index))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_Ceiling(&mut self) -> quest_hook::libil2cpp::Result<*mut OVRScenePlane> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OVRScenePlane = __cordl_object.invoke("get_Ceiling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Floor(&mut self) -> quest_hook::libil2cpp::Result<*mut OVRScenePlane> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OVRScenePlane = __cordl_object.invoke("get_Floor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Walls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut OVRScenePlane>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<*mut OVRScenePlane> = __cordl_object
            .invoke("get_Walls", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Ceiling(
        &mut self,
        value: *mut OVRScenePlane,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Ceiling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Floor(
        &mut self,
        value: *mut OVRScenePlane,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Floor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Walls(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Walls", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl quest_hook::libil2cpp::ObjectType for OVRSceneRoom {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
