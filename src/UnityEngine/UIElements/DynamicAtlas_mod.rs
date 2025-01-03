#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicAtlas {
    __cordl_parent: crate::UnityEngine::UIElements::AtlasBase,
    pub m_Database: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::Texture,
        *mut crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo,
    >,
    pub m_PointPage: *mut crate::UnityEngine::UIElements::DynamicAtlasPage,
    pub m_BilinearPage: *mut crate::UnityEngine::UIElements::DynamicAtlasPage,
    pub m_ColorSpace: crate::UnityEngine::ColorSpace,
    pub m_Panels: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::IPanel,
    >,
    pub m_MinAtlasSize: i32,
    pub m_MaxAtlasSize: i32,
    pub m_MaxSubTextureSize: i32,
    pub m_ActiveFilters: crate::UnityEngine::UIElements::DynamicAtlasFilters,
    pub m_CustomFilter: *mut crate::UnityEngine::UIElements::DynamicAtlasCustomFilter,
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DynamicAtlas =>
    "UnityEngine.UIElements"."DynamicAtlas"
);
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DynamicAtlas {
    type Target = crate::UnityEngine::UIElements::AtlasBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DynamicAtlas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas")]
impl crate::UnityEngine::UIElements::DynamicAtlas {
    #[cfg(feature = "UnityEngine+UIElements+DynamicAtlas+TextureInfo")]
    pub type TextureInfo = crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo;
    pub fn DestroyPages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyPages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitPages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitPages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTextureFormatSupported(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTextureFormatSupported", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTextureValid(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        atlasFilterMode: crate::UnityEngine::FilterMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsTextureValid", (texture, atlasFilterMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnAssignedToPanel(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAssignedToPanel", (panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRemovedFromPanel(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRemovedFromPanel", (panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUpdateDynamicTextures(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdateDynamicTextures", (panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReturnAtlas(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        atlas: crate::UnityEngine::UIElements::TextureId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReturnAtlas", (ve, src, atlas))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetAtlas(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        atlas: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TextureId,
        >,
        atlasRect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetAtlas", (ve, src, atlas, atlasRect))?;
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
    pub fn get_defaultFilters() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DynamicAtlasFilters,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::DynamicAtlasFilters = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultFilters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxSubTextureSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxSubTextureSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_activeFilters(
        &mut self,
        value: crate::UnityEngine::UIElements::DynamicAtlasFilters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_activeFilters", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_customFilter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DynamicAtlasCustomFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_customFilter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxAtlasSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxAtlasSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxSubTextureSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxSubTextureSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minAtlasSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minAtlasSize", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::DynamicAtlas {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas+TextureInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicAtlas_TextureInfo {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo,
    >,
    pub page: *mut crate::UnityEngine::UIElements::DynamicAtlasPage,
    pub counter: i32,
    pub alloc: crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D,
    pub rect: crate::UnityEngine::RectInt,
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas+TextureInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DynamicAtlas_TextureInfo => "UnityEngine.UIElements"
    ."DynamicAtlas/TextureInfo"
);
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas+TextureInfo")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas+TextureInfo")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas+TextureInfo")]
impl crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        info: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reset", (info))?;
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
#[cfg(feature = "UnityEngine+UIElements+DynamicAtlas+TextureInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DynamicAtlas_TextureInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
