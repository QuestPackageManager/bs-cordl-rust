#[cfg(feature = "MultiplayerEnvironmentResizeController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerEnvironmentResizeController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _platformEnd: *mut crate::UnityEngine::Transform,
    pub _resizeData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeData,
    >,
    pub _centerResizeController: *mut crate::GlobalNamespace::MultiplayerCenterResizeController,
    pub _beatmapObjectSpawnCenter: *mut crate::GlobalNamespace::BeatmapObjectSpawnCenter,
    pub resizingDidFinishEvent: *mut crate::System::Action,
    pub _isResizingFinished: bool,
    pub _edgeDistanceFromCenterFound: bool,
    pub _spawnCenterDistanceFound: bool,
}
#[cfg(feature = "MultiplayerEnvironmentResizeController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerEnvironmentResizeController => ""
    ."MultiplayerEnvironmentResizeController"
);
#[cfg(feature = "MultiplayerEnvironmentResizeController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerEnvironmentResizeController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerEnvironmentResizeController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerEnvironmentResizeController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerEnvironmentResizeController")]
impl crate::GlobalNamespace::MultiplayerEnvironmentResizeController {
    #[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeData")]
    pub type ResizeData = crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeData;
    #[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeType")]
    pub type ResizeType = crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeType;
    pub fn HandleEdgeDistanceFromCenterWasCalculated(
        &mut self,
        edgeDistanceFromCenter: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleEdgeDistanceFromCenterWasCalculated",
                (edgeDistanceFromCenter),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSpawnCenterDistanceWasFound(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpawnCenterDistanceWasFound", (distance))?;
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
    pub fn Resize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryResize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryResize", ())?;
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
    pub fn add_resizingDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_resizingDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isResizingFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isResizingFinished", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_resizingDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_resizingDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerEnvironmentResizeController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerEnvironmentResizeController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerEnvironmentResizeController_ResizeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _resizeType: crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeType,
    pub _offset: f32,
    pub _lights: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::TubeBloomPrePassLight,
    >,
    pub _otherTransforms: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Transform,
    >,
}
#[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeData => ""
    ."MultiplayerEnvironmentResizeController/ResizeData"
);
#[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeData")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeData")]
impl crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeData {
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
    pub fn get_lights(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::TubeBloomPrePassLight,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::TubeBloomPrePassLight,
        > = __cordl_object.invoke("get_lights", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_offset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_offset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_otherTransforms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Transform,
        > = __cordl_object.invoke("get_otherTransforms", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_resizeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeType = __cordl_object
            .invoke("get_resizeType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerEnvironmentResizeController_ResizeType {
    Length = 2i32,
    None = 0i32,
    Position = 1i32,
}
#[cfg(feature = "MultiplayerEnvironmentResizeController+ResizeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerEnvironmentResizeController_ResizeType => ""
    ."MultiplayerEnvironmentResizeController/ResizeType"
);
