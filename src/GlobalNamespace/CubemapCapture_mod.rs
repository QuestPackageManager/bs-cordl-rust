#[cfg(feature = "CubemapCapture")]
#[repr(C)]
#[derive(Debug)]
pub struct CubemapCapture {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _camera: *mut crate::UnityEngine::Camera,
    pub _triggerKey: crate::UnityEngine::InputSystem::Key,
    pub _cubemapSize: i32,
    pub _saveDirectoryAbsolutePath: *mut crate::System::String,
    pub _imageType: crate::GlobalNamespace::CubemapCapture_ImageType,
    pub _jpegCompression: i32,
    pub _exrFlags: crate::UnityEngine::Texture2D_EXRFlags,
    pub _faces: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::CubemapFace>,
    pub _faceAngles: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "CubemapCapture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CubemapCapture => ""
    ."CubemapCapture"
);
#[cfg(feature = "CubemapCapture")]
impl std::ops::Deref for crate::GlobalNamespace::CubemapCapture {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CubemapCapture")]
impl std::ops::DerefMut for crate::GlobalNamespace::CubemapCapture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CubemapCapture")]
impl crate::GlobalNamespace::CubemapCapture {
    #[cfg(feature = "CubemapCapture+ImageType")]
    pub type ImageType = crate::GlobalNamespace::CubemapCapture_ImageType;
    pub fn GenerateAndSaveCubemapTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateAndSaveCubemapTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RenderCubemapTexture(
        &mut self,
        cubemap: *mut crate::UnityEngine::Cubemap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderCubemapTexture", (cubemap))?;
        Ok(__cordl_ret)
    }
    pub fn SaveCubemapTexture(
        &mut self,
        cubemap: *mut crate::UnityEngine::Cubemap,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveCubemapTexture", (cubemap, path))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeTextureToByteArray(
        &mut self,
        texture: *mut crate::UnityEngine::Texture2D,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("SerializeTextureToByteArray", (texture))?;
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
#[cfg(feature = "CubemapCapture")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CubemapCapture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CubemapCapture+ImageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubemapCapture_ImageType {
    EXR = 3i32,
    JPEG = 1i32,
    PNG = 0i32,
    TGA = 2i32,
}
#[cfg(feature = "CubemapCapture+ImageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CubemapCapture_ImageType => ""
    ."CubemapCapture/ImageType"
);
