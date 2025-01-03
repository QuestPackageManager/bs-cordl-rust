#[cfg(feature = "CubemapTest")]
#[repr(C)]
#[derive(Debug)]
pub struct CubemapTest {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _cubemapMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _flatMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _cubemapRenderTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
    pub _downsampledCubemapRenderTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
    pub _cubemapFlatTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
}
#[cfg(feature = "CubemapTest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CubemapTest => ""."CubemapTest"
);
#[cfg(feature = "CubemapTest")]
impl std::ops::Deref for crate::GlobalNamespace::CubemapTest {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CubemapTest")]
impl std::ops::DerefMut for crate::GlobalNamespace::CubemapTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CubemapTest")]
impl crate::GlobalNamespace::CubemapTest {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "CubemapTest")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CubemapTest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
