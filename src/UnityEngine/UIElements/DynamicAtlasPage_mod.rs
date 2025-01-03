#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasPage")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicAtlasPage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _textureId_k__BackingField: crate::UnityEngine::UIElements::TextureId,
    pub _atlas_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
    pub _format_k__BackingField: crate::UnityEngine::RenderTextureFormat,
    pub _filterMode_k__BackingField: crate::UnityEngine::FilterMode,
    pub _minSize_k__BackingField: crate::UnityEngine::Vector2Int,
    pub _maxSize_k__BackingField: crate::UnityEngine::Vector2Int,
    pub m_1Padding: i32,
    pub m_2Padding: i32,
    pub m_Allocator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::Allocator2D,
    >,
    pub m_Blitter: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::TextureBlitter,
    >,
    pub m_CurrentSize: crate::UnityEngine::Vector2Int,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasPage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DynamicAtlasPage =>
    "UnityEngine.UIElements"."DynamicAtlasPage"
);
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasPage")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DynamicAtlasPage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasPage")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DynamicAtlasPage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasPage")]
impl crate::UnityEngine::UIElements::DynamicAtlasPage {
    pub fn Commit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Commit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateAtlasTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = __cordl_object
            .invoke("CreateAtlasTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        format: crate::UnityEngine::RenderTextureFormat,
        filterMode: crate::UnityEngine::FilterMode,
        minSize: crate::UnityEngine::Vector2Int,
        maxSize: crate::UnityEngine::Vector2Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (format, filterMode, minSize, maxSize))?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (alloc))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAdd(
        &mut self,
        image: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        alloc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D,
        >,
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryAdd", (image, alloc, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        image: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (image, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAtlasTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAtlasTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        format: crate::UnityEngine::RenderTextureFormat,
        filterMode: crate::UnityEngine::FilterMode,
        minSize: crate::UnityEngine::Vector2Int,
        maxSize: crate::UnityEngine::Vector2Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (format, filterMode, minSize, maxSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = __cordl_object
            .invoke("get_atlas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_filterMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::FilterMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::FilterMode = __cordl_object
            .invoke("get_filterMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureFormat> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderTextureFormat = __cordl_object
            .invoke("get_format", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textureId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextureId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::TextureId = __cordl_object
            .invoke("get_textureId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_atlas(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_atlas", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_textureId(
        &mut self,
        value: crate::UnityEngine::UIElements::TextureId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textureId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasPage")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DynamicAtlasPage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasPage")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::DynamicAtlasPage {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlasPage")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::DynamicAtlasPage {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
