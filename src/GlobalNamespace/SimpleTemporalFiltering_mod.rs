#[cfg(feature = "SimpleTemporalFiltering")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleTemporalFiltering {
    __cordl_parent: crate::System::Object,
    pub _temporalFilteringTextures: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::RenderTexture,
    >,
    pub _prevTemporalFilteringTextureIdx: i32,
    pub _temporalFilteringMaterial: *mut crate::UnityEngine::Material,
    pub _bufferTexID: i32,
}
#[cfg(feature = "SimpleTemporalFiltering")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SimpleTemporalFiltering => ""."SimpleTemporalFiltering"
);
#[cfg(feature = "SimpleTemporalFiltering")]
impl std::ops::Deref for SimpleTemporalFiltering {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SimpleTemporalFiltering")]
impl std::ops::DerefMut for SimpleTemporalFiltering {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SimpleTemporalFiltering")]
impl SimpleTemporalFiltering {
    pub fn FilterTexture(
        &mut self,
        src: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RenderTexture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RenderTexture = __cordl_object
            .invoke("FilterTexture", (src))?;
        Ok(__cordl_ret)
    }
    pub fn CreateRenderTexturesIfNeeded(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateRenderTexturesIfNeeded", (width, height))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SimpleTemporalFiltering")]
impl quest_hook::libil2cpp::ObjectType for SimpleTemporalFiltering {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
