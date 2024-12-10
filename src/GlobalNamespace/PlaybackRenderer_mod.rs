#[cfg(feature = "PlaybackRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlaybackRenderer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub texturesReadyEvent: *mut crate::System::Action,
    pub _clearBackgroundShader: *mut crate::UnityEngine::Shader,
    pub _isSetup: bool,
    pub _hmd: *mut crate::UnityEngine::Transform,
    pub _camera: *mut crate::UnityEngine::Camera,
    pub _cameraCalibration: *mut crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration,
    pub _clipQuad: *mut crate::UnityEngine::GameObject,
    pub _clipMaterial: *mut crate::UnityEngine::Material,
    pub _screenshots: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
    >,
}
#[cfg(feature = "PlaybackRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlaybackRenderer => ""
    ."PlaybackRenderer"
);
#[cfg(feature = "PlaybackRenderer")]
impl std::ops::Deref for crate::GlobalNamespace::PlaybackRenderer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlaybackRenderer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlaybackRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlaybackRenderer")]
impl crate::GlobalNamespace::PlaybackRenderer {
    #[cfg(feature = "PlaybackRenderer+PlaybackScreenshot")]
    pub type PlaybackScreenshot = crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot;
    pub fn CreateClipQuad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateClipQuad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTextures(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateTextures", (width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDistanceToHMD(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetDistanceToHMD", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitCamera(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cameraCalibration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitCamera", (camera, cameraCalibration))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OrientClipQuad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OrientClipQuad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderBackground(
        &mut self,
        screenshot: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderBackground", (screenshot))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderForeground(
        &mut self,
        screenshot: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderForeground", (screenshot))?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        hmdCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cameraCalibration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingData_ExternalCameraCalibration,
        >,
        textureWidth: i32,
        textureHeight: i32,
        screenshots: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Setup",
                (
                    hmdCamera,
                    camera,
                    cameraCalibration,
                    textureWidth,
                    textureHeight,
                    screenshots,
                ),
            )?;
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
    pub fn add_texturesReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_texturesReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screenshots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
            >,
        > = __cordl_object.invoke("get_screenshots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_texturesReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_texturesReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlaybackRenderer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlaybackRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlaybackRenderer+PlaybackScreenshot")]
#[repr(C)]
#[derive(Debug)]
pub struct PlaybackRenderer_PlaybackScreenshot {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _layerMask: crate::UnityEngine::LayerMask,
    pub _type: crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
    pub _texture: *mut crate::UnityEngine::RenderTexture,
    pub _path: *mut quest_hook::libil2cpp::Il2CppString,
    pub _backgroundColor: crate::UnityEngine::Color,
}
#[cfg(feature = "PlaybackRenderer+PlaybackScreenshot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot => ""
    ."PlaybackRenderer/PlaybackScreenshot"
);
#[cfg(feature = "PlaybackRenderer+PlaybackScreenshot")]
impl std::ops::Deref for crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlaybackRenderer+PlaybackScreenshot")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlaybackRenderer+PlaybackScreenshot")]
impl crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot {
    #[cfg(feature = "PlaybackRenderer+PlaybackScreenshot+Type")]
    pub type Type = crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type;
    pub fn CreateTexture(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateTexture", (width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        layerMask: crate::UnityEngine::LayerMask,
        _cordl_type: crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
        backgroundColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, layerMask, _cordl_type, backgroundColor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        layerMask: crate::UnityEngine::LayerMask,
        _cordl_type: crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
        backgroundColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, layerMask, _cordl_type, backgroundColor))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_backgroundColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layerMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("get_layerMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_path", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_texture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = __cordl_object
            .invoke("get_texture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_path(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_path", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlaybackRenderer+PlaybackScreenshot")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlaybackRenderer+PlaybackScreenshot+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackScreenshot_PlaybackRenderer_Type {
    Background = 2i32,
    Foreground = 1i32,
}
#[cfg(feature = "PlaybackRenderer+PlaybackScreenshot+Type")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type => ""
    ."PlaybackRenderer/PlaybackScreenshot/Type"
);
