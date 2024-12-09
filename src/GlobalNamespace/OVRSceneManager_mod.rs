#[cfg(feature = "OVRSceneManager")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub PlanePrefab: *mut crate::GlobalNamespace::OVRSceneAnchor,
    pub VolumePrefab: *mut crate::GlobalNamespace::OVRSceneAnchor,
    pub PrefabOverrides: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRScenePrefabOverride,
    >,
    pub VerboseLogging: bool,
    pub MaxSceneAnchorUpdatesPerFrame: i32,
    pub _initialAnchorParent: *mut crate::UnityEngine::Transform,
    pub SceneModelLoadedSuccessfully: *mut crate::System::Action,
    pub NoSceneModelToLoad: *mut crate::System::Action,
    pub SceneCaptureReturnedWithoutError: *mut crate::System::Action,
    pub UnexpectedErrorWithSceneCapture: *mut crate::System::Action,
    pub NewSceneModelAvailable: *mut crate::System::Action,
    pub RoomLayout: *mut crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation,
    pub _sceneCaptureRequestId: u64,
    pub _cameraRig: *mut crate::GlobalNamespace::OVRCameraRig,
    pub _sceneAnchorUpdateIndex: i32,
    pub _roomCounter: i32,
    pub _onAnchorsFetchCompleted: *mut crate::System::Action_2<
        bool,
        *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::OVRAnchor,
        >,
    >,
    pub _hasLoadedScene: bool,
}
#[cfg(feature = "OVRSceneManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSceneManager => ""
    ."OVRSceneManager"
);
#[cfg(feature = "OVRSceneManager")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSceneManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneManager")]
impl crate::GlobalNamespace::OVRSceneManager {
    #[cfg(feature = "OVRSceneManager+Classification")]
    pub type Classification = crate::GlobalNamespace::OVRSceneManager_Classification;
    #[cfg(feature = "OVRSceneManager+Development")]
    pub type Development = crate::GlobalNamespace::OVRSceneManager_Development;
    #[cfg(feature = "OVRSceneManager+LogForwarder")]
    pub type LogForwarder = crate::GlobalNamespace::OVRSceneManager_LogForwarder;
    #[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
    pub type RoomLayoutInformation = crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation;
    #[cfg(feature = "OVRSceneManager+_OnApplicationPause_d__28")]
    pub type _OnApplicationPause_d__28 = crate::GlobalNamespace::OVRSceneManager__OnApplicationPause_d__28;
    #[cfg(feature = "OVRSceneManager+_QueryForExistingAnchorsTransform_d__29")]
    pub type _QueryForExistingAnchorsTransform_d__29 = crate::GlobalNamespace::OVRSceneManager__QueryForExistingAnchorsTransform_d__29;
    #[cfg(feature = "OVRSceneManager+__c__DisplayClass37_0")]
    pub type __c__DisplayClass37_0 = crate::GlobalNamespace::OVRSceneManager___c__DisplayClass37_0;
    #[cfg(feature = "OVRSceneManager+__c__DisplayClass40_0")]
    pub type __c__DisplayClass40_0 = crate::GlobalNamespace::OVRSceneManager___c__DisplayClass40_0;
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
    pub fn DestroyExistingAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyExistingAnchors", ())?;
        Ok(__cordl_ret)
    }
    pub fn DoesRoomSetupExist(
        &mut self,
        requestedAnchorClassifications: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = __cordl_object
            .invoke("DoesRoomSetupExist", (requestedAnchorClassifications))?;
        Ok(__cordl_ret)
    }
    pub fn GetRoomLayoutInformation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation = __cordl_object
            .invoke("GetRoomLayoutInformation", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateSceneAnchor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
        prefab: *mut crate::GlobalNamespace::OVRSceneAnchor,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::OVRSceneAnchor> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OVRSceneAnchor = __cordl_object
            .invoke("InstantiateSceneAnchor", (anchor, prefab))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateSceneRooms(
        &mut self,
        roomLayoutAnchors: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::OVRAnchor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstantiateSceneRooms", (roomLayoutAnchors))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneModel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LoadSceneModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OVRManager_SceneCaptureComplete(
        &mut self,
        requestId: u64,
        result: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OVRManager_SceneCaptureComplete", (requestId, result))?;
        Ok(__cordl_ret)
    }
    pub fn OnAnchorsFetchCompleted(
        &mut self,
        success: bool,
        roomLayoutAnchors: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::OVRAnchor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAnchorsFetchCompleted", (success, roomLayoutAnchors))?;
        Ok(__cordl_ret)
    }
    pub fn OnApplicationPause(
        &mut self,
        isPaused: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationPause", (isPaused))?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnSceneRoomLoadCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSceneRoomLoadCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn QueryForExistingAnchorsTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueryForExistingAnchorsTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn RequestSceneCapture_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RequestSceneCapture", ())?;
        Ok(__cordl_ret)
    }
    pub fn RequestSceneCapture_IEnumerable_1_1(
        &mut self,
        requestedAnchorClassifications: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RequestSceneCapture", (requestedAnchorClassifications))?;
        Ok(__cordl_ret)
    }
    pub fn RequestSceneCapture_String2(
        &mut self,
        requestString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RequestSceneCapture", (requestString))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSomeSceneAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSomeSceneAnchors", ())?;
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
    pub fn get_InitialAnchorParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_InitialAnchorParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Verbose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::GlobalNamespace::OVRSceneManager_LogForwarder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::GlobalNamespace::OVRSceneManager_LogForwarder,
        > = __cordl_object.invoke("get_Verbose", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_InitialAnchorParent(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InitialAnchorParent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSceneManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRSceneManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSceneManager+Classification")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneManager_Classification {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRSceneManager+Classification")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSceneManager_Classification
    => ""."OVRSceneManager/Classification"
);
#[cfg(feature = "OVRSceneManager+Classification")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneManager_Classification {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneManager+Classification")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSceneManager_Classification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneManager+Classification")]
impl crate::GlobalNamespace::OVRSceneManager_Classification {
    pub const Bed: &'static str = "BED";
    pub const Ceiling: &'static str = "CEILING";
    pub const Couch: &'static str = "COUCH";
    pub const Desk: &'static str = "DESK";
    pub const DoorFrame: &'static str = "DOOR_FRAME";
    pub const Floor: &'static str = "FLOOR";
    pub const GlobalMesh: &'static str = "GLOBAL_MESH";
    pub const InvisibleWallFace: &'static str = "INVISIBLE_WALL_FACE";
    pub const Lamp: &'static str = "LAMP";
    pub const Other: &'static str = "OTHER";
    pub const Plant: &'static str = "PLANT";
    pub const Screen: &'static str = "SCREEN";
    pub const Storage: &'static str = "STORAGE";
    pub const Table: &'static str = "TABLE";
    pub const WallArt: &'static str = "WALL_ART";
    pub const WallFace: &'static str = "WALL_FACE";
    pub const WindowFrame: &'static str = "WINDOW_FRAME";
}
#[cfg(feature = "OVRSceneManager+Classification")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSceneManager_Classification {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSceneManager+Development")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneManager_Development {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRSceneManager+Development")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSceneManager_Development =>
    ""."OVRSceneManager/Development"
);
#[cfg(feature = "OVRSceneManager+Development")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneManager_Development {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneManager+Development")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSceneManager_Development {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneManager+Development")]
impl crate::GlobalNamespace::OVRSceneManager_Development {}
#[cfg(feature = "OVRSceneManager+Development")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSceneManager_Development {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSceneManager+LogForwarder")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRSceneManager_LogForwarder {}
#[cfg(feature = "OVRSceneManager+LogForwarder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSceneManager_LogForwarder =>
    ""."OVRSceneManager/LogForwarder"
);
#[cfg(feature = "OVRSceneManager+LogForwarder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSceneManager_LogForwarder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSceneManager+LogForwarder")]
impl crate::GlobalNamespace::OVRSceneManager_LogForwarder {
    pub fn Log(
        &mut self,
        context: *mut crate::System::String,
        message: *mut crate::System::String,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Log",
            (context, message, gameObject),
        )?;
        Ok(__cordl_ret)
    }
    pub fn LogError(
        &mut self,
        context: *mut crate::System::String,
        message: *mut crate::System::String,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LogError",
            (context, message, gameObject),
        )?;
        Ok(__cordl_ret)
    }
    pub fn LogWarning(
        &mut self,
        context: *mut crate::System::String,
        message: *mut crate::System::String,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LogWarning",
            (context, message, gameObject),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneManager_RoomLayoutInformation {
    __cordl_parent: crate::System::Object,
    pub Floor: *mut crate::GlobalNamespace::OVRScenePlane,
    pub Ceiling: *mut crate::GlobalNamespace::OVRScenePlane,
    pub Walls: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRScenePlane,
    >,
}
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSceneManager_RoomLayoutInformation => ""
    ."OVRSceneManager/RoomLayoutInformation"
);
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
impl crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
