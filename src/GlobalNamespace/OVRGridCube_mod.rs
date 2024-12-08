#[cfg(feature = "OVRGridCube")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGridCube {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub GridKey: crate::UnityEngine::KeyCode,
    pub CubeGrid: *mut crate::UnityEngine::GameObject,
    pub CubeGridOn: bool,
    pub CubeSwitchColorOld: bool,
    pub CubeSwitchColor: bool,
    pub gridSizeX: i32,
    pub gridSizeY: i32,
    pub gridSizeZ: i32,
    pub gridScale: f32,
    pub cubeScale: f32,
    pub CameraController: *mut crate::GlobalNamespace::OVRCameraRig,
}
#[cfg(feature = "OVRGridCube")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRGridCube => ""."OVRGridCube"
);
#[cfg(feature = "OVRGridCube")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGridCube {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGridCube")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRGridCube {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGridCube")]
impl crate::GlobalNamespace::OVRGridCube {
    pub fn CreateCubeGrid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateCubeGrid", ())?;
        Ok(__cordl_ret)
    }
    pub fn CubeGridSwitchColor(
        &mut self,
        CubeSwitchColor: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CubeGridSwitchColor", (CubeSwitchColor))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetOVRCameraController(
        &mut self,
        cameraController: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::OVRCameraRig,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOVRCameraController", (cameraController))?;
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
    pub fn UpdateCubeGrid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCubeGrid", ())?;
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
}
#[cfg(feature = "OVRGridCube")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRGridCube {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
