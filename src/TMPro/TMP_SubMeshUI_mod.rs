#[cfg(feature = "TMPro+TMP_SubMeshUI")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_SubMeshUI {
    __cordl_parent: crate::UnityEngine::UI::MaskableGraphic,
    pub m_fontAsset: *mut crate::TMPro::TMP_FontAsset,
    pub m_spriteAsset: *mut crate::TMPro::TMP_SpriteAsset,
    pub m_material: *mut crate::UnityEngine::Material,
    pub m_sharedMaterial: *mut crate::UnityEngine::Material,
    pub m_fallbackMaterial: *mut crate::UnityEngine::Material,
    pub m_fallbackSourceMaterial: *mut crate::UnityEngine::Material,
    pub m_isDefaultMaterial: bool,
    pub m_padding: f32,
    pub m_mesh: *mut crate::UnityEngine::Mesh,
    pub m_TextComponent: *mut crate::TMPro::TextMeshProUGUI,
    pub m_isRegisteredForEvents: bool,
    pub m_materialDirty: bool,
    pub m_materialReferenceIndex: i32,
    pub m_RootCanvasTransform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "TMPro+TMP_SubMeshUI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_SubMeshUI => "TMPro"."TMP_SubMeshUI"
);
#[cfg(feature = "TMPro+TMP_SubMeshUI")]
impl std::ops::Deref for crate::TMPro::TMP_SubMeshUI {
    type Target = crate::UnityEngine::UI::MaskableGraphic;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_SubMeshUI")]
impl std::ops::DerefMut for crate::TMPro::TMP_SubMeshUI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_SubMeshUI")]
impl crate::TMPro::TMP_SubMeshUI {
    pub fn set_fallbackSourceMaterial(
        &mut self,
        value: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fallbackSourceMaterial", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetAllDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAllDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecalculateClipping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateClipping", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_materialForRendering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_materialForRendering", ())?;
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
    pub fn set_mesh(
        &mut self,
        value: *mut crate::UnityEngine::Mesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mesh", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetLayoutDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_material(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_material", ())?;
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
    pub fn set_fallbackMaterial(
        &mut self,
        value: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fallbackMaterial", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetVerticesDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVerticesDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spriteAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_SpriteAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_SpriteAsset = __cordl_object
            .invoke("get_spriteAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_padding(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_padding", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_sharedMaterial(
        &mut self,
        value: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sharedMaterial", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGeometry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGeometry", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_spriteAsset(
        &mut self,
        value: *mut crate::TMPro::TMP_SpriteAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_textComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_Text> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_Text = __cordl_object
            .invoke("get_textComponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fallbackMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_fallbackMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn Cull(
        &mut self,
        clipRect: crate::UnityEngine::Rect,
        validRect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cull", (clipRect, validRect))?;
        Ok(__cordl_ret)
    }
    pub fn SetSharedMaterial(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSharedMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn GetSharedMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("GetSharedMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnTransformParentChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTransformParentChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sharedMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_sharedMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fallbackSourceMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_fallbackSourceMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn Rebuild(
        &mut self,
        update: crate::UnityEngine::UI::CanvasUpdate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rebuild", (update))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMeshPadding(
        &mut self,
        isExtraPadding: bool,
        isUsingBold: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMeshPadding", (isExtraPadding, isUsingBold))?;
        Ok(__cordl_ret)
    }
    pub fn GetMaterial_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("GetMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMaterial_Material1(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("GetMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn get_isDefaultMaterial(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDefaultMaterial", ())?;
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
    pub fn GetPaddingForMaterial_0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPaddingForMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPaddingForMaterial_Material1(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetPaddingForMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn get_fontAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_FontAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_FontAsset = __cordl_object
            .invoke("get_fontAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_material(
        &mut self,
        value: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_material", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isDefaultMaterial(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDefaultMaterial", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_padding(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_padding", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetPivotDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPivotDirty", ())?;
        Ok(__cordl_ret)
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
    pub fn CreateMaterialInstance(
        &mut self,
        source: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("CreateMaterialInstance", (source))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetMaterialDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMaterialDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRootCanvasTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("GetRootCanvasTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mainTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture = __cordl_object
            .invoke("get_mainTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_fontAsset(
        &mut self,
        value: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetModifiedMaterial(
        &mut self,
        baseMaterial: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("GetModifiedMaterial", (baseMaterial))?;
        Ok(__cordl_ret)
    }
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Mesh> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Mesh = __cordl_object
            .invoke("get_mesh", ())?;
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
#[cfg(feature = "TMPro+TMP_SubMeshUI")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_SubMeshUI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
