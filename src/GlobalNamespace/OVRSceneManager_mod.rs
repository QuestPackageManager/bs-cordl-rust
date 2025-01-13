#[cfg(feature = "OVRSceneManager")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub PlanePrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSceneAnchor>,
    pub VolumePrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSceneAnchor>,
    pub PrefabOverrides: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePrefabOverride>,
        >,
    >,
    pub VerboseLogging: bool,
    pub MaxSceneAnchorUpdatesPerFrame: i32,
    pub _initialAnchorParent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub SceneModelLoadedSuccessfully: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub NoSceneModelToLoad: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub SceneCaptureReturnedWithoutError: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub UnexpectedErrorWithSceneCapture: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub NewSceneModelAvailable: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub RoomLayout: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation,
    >,
    pub _sceneCaptureRequestId: u64,
    pub _cameraRig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>,
    pub _sceneAnchorUpdateIndex: i32,
    pub _roomCounter: i32,
    pub _onAnchorsFetchCompleted: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            bool,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::GlobalNamespace::OVRAnchor,
                >,
            >,
        >,
    >,
    pub _hasLoadedScene: bool,
}
#[cfg(feature = "OVRSceneManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRSceneManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRSceneManager";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn CheckClassificationsInRooms(
        success: bool,
        rooms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
        requestedAnchorClassifications: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        task: crate::GlobalNamespace::OVRTask_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckClassificationsInRooms",
                (success, rooms, requestedAnchorClassifications, task),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIfAnchorsContainClassifications(
        success: bool,
        roomAnchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
        requestedAnchorClassifications: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        task: crate::GlobalNamespace::OVRTask_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CheckIfAnchorsContainClassifications",
                (success, roomAnchors, requestedAnchorClassifications, task),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIfClassificationsAreValid(
        requestedAnchorClassifications: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIfClassificationsAreValid", (requestedAnchorClassifications))?;
        Ok(__cordl_ret.into())
    }
    pub fn CollectLabelsFromAnchors(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
        labels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CollectLabelsFromAnchors", (anchors, labels))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyExistingAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyExistingAnchors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoesRoomSetupExist(
        &mut self,
        requestedAnchorClassifications: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = __cordl_object
            .invoke("DoesRoomSetupExist", (requestedAnchorClassifications))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRoomLayoutInformation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation,
        > = __cordl_object.invoke("GetRoomLayoutInformation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUuidsToQuery(
        anchor: crate::GlobalNamespace::OVRAnchor,
        uuidsToQuery: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<crate::System::Guid>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUuidsToQuery", (anchor, uuidsToQuery))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateSceneAnchor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
        prefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSceneAnchor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSceneAnchor>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSceneAnchor,
        > = __cordl_object.invoke("InstantiateSceneAnchor", (anchor, prefab))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateSceneRooms(
        &mut self,
        roomLayoutAnchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstantiateSceneRooms", (roomLayoutAnchors))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneModel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LoadSceneModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn OnAnchorsFetchCompleted(
        &mut self,
        success: bool,
        roomLayoutAnchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAnchorsFetchCompleted", (success, roomLayoutAnchors))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSceneRoomLoadCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSceneRoomLoadCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTrackingSpaceChanged(
        trackingSpace: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnTrackingSpaceChanged", (trackingSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryForExistingAnchorsTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueryForExistingAnchorsTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestSceneCapture_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RequestSceneCapture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestSceneCapture_IEnumerable_1_1(
        &mut self,
        requestedAnchorClassifications: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RequestSceneCapture", (requestedAnchorClassifications))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestSceneCapture_Il2CppString2(
        &mut self,
        requestString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RequestSceneCapture", (requestString))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAllSceneAnchors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateAllSceneAnchors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSomeSceneAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSomeSceneAnchors", ())?;
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
    pub fn get_InitialAnchorParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("get_InitialAnchorParent", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_InitialAnchorParent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InitialAnchorParent", (value))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRSceneManager+Classification")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRSceneManager_Classification {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRSceneManager/Classification";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRSceneManager+Classification")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneManager_Classification {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_List() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_List", ())?;
        Ok(__cordl_ret.into())
    }
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRSceneManager+Development")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRSceneManager_Development {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRSceneManager/Development";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRSceneManager+Development")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneManager_Development {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRSceneManager_Development {
    pub fn Log(
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (context, message, gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogError", (context, message, gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogWarning", (context, message, gameObject))?;
        Ok(__cordl_ret.into())
    }
}
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRSceneManager_LogForwarder {}
#[cfg(feature = "OVRSceneManager+LogForwarder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRSceneManager_LogForwarder {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LogForwarder";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRSceneManager+LogForwarder")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRSceneManager_LogForwarder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRSceneManager+LogForwarder")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRSceneManager_LogForwarder {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRSceneManager+LogForwarder")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRSceneManager_LogForwarder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRSceneManager+LogForwarder")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRSceneManager_LogForwarder {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Log",
            (context, message, gameObject),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LogError",
            (context, message, gameObject),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LogWarning",
            (context, message, gameObject),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneManager_RoomLayoutInformation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Floor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    pub Ceiling: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    pub Walls: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
        >,
    >,
}
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRSceneManager/RoomLayoutInformation";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRSceneManager+RoomLayoutInformation")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneManager_RoomLayoutInformation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
