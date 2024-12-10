#[cfg(feature = "OVRSceneRoom")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneRoom {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _Floor_k__BackingField: *mut crate::GlobalNamespace::OVRScenePlane,
    pub _Ceiling_k__BackingField: *mut crate::GlobalNamespace::OVRScenePlane,
    pub _Walls_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::OVRScenePlane,
    >,
    pub _walls: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRScenePlane,
    >,
    pub _orderedRoomGuids: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::Guid,
        i32,
    >,
    pub _wallOrderComparer: *mut crate::System::Comparison_1<
        *mut crate::GlobalNamespace::OVRScenePlane,
    >,
    pub _sceneAnchor: *mut crate::GlobalNamespace::OVRSceneAnchor,
    pub _sceneManager: *mut crate::GlobalNamespace::OVRSceneManager,
    pub _uuidToQuery: *mut crate::System::Collections::Generic::HashSet_1<
        crate::System::Guid,
    >,
    pub _roomAnchors: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::OVRAnchor,
    >,
    pub _taskCount: i32,
    pub _onFetchAnchorsCompleted: *mut crate::System::Action_1<bool>,
    pub _onAnchorLocalizationCompleted: *mut crate::System::Action_2<
        bool,
        crate::GlobalNamespace::OVRAnchor,
    >,
}
#[cfg(feature = "OVRSceneRoom")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSceneRoom => ""
    ."OVRSceneRoom"
);
#[cfg(feature = "OVRSceneRoom")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneRoom {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSceneRoom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl crate::GlobalNamespace::OVRSceneRoom {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUuidsToQuery(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUuidsToQuery", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IOVRSceneComponent_Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IOVRSceneComponent.Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRoom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRoom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn OnLocalizationCompleted(
        &mut self,
        success: bool,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLocalizationCompleted", (success, anchor))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRoomInformation(
        &mut self,
        plane: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRoomInformation", (plane))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__24_0(
        &mut self,
        planeA: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
        planeB: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("<Awake>b__24_0", (planeA, planeB))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_g__TryGetUuid_24_1(
        &mut self,
        plane: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<Awake>g__TryGetUuid|24_1", (plane, index))?;
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
    pub fn get_Ceiling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRScenePlane,
        > = __cordl_object.invoke("get_Ceiling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Floor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRScenePlane,
        > = __cordl_object.invoke("get_Floor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Walls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::OVRScenePlane,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::OVRScenePlane,
            >,
        > = __cordl_object.invoke("get_Walls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Ceiling(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Ceiling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Floor(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Floor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Walls(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::OVRScenePlane,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Walls", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRSceneRoom {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl AsRef<crate::GlobalNamespace::IOVRSceneComponent>
for crate::GlobalNamespace::OVRSceneRoom {
    fn as_ref(&self) -> &crate::GlobalNamespace::IOVRSceneComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl AsMut<crate::GlobalNamespace::IOVRSceneComponent>
for crate::GlobalNamespace::OVRSceneRoom {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IOVRSceneComponent {
        unsafe { std::mem::transmute(self) }
    }
}
