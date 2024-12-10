#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
#[repr(C)]
#[derive(Debug)]
pub struct MaskableGraphic {
    __cordl_parent: crate::UnityEngine::UI::Graphic,
    pub m_ShouldRecalculateStencil: bool,
    pub m_MaskMaterial: *mut crate::UnityEngine::Material,
    pub m_ParentMask: *mut crate::UnityEngine::UI::RectMask2D,
    pub m_Maskable: bool,
    pub m_IsMaskingGraphic: bool,
    pub m_IncludeForMasking: bool,
    pub m_OnCullStateChanged: *mut crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
    pub m_ShouldRecalculate: bool,
    pub m_StencilValue: i32,
    pub m_Corners: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::MaskableGraphic =>
    "UnityEngine.UI"."MaskableGraphic"
);
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl std::ops::Deref for crate::UnityEngine::UI::MaskableGraphic {
    type Target = crate::UnityEngine::UI::Graphic;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl std::ops::DerefMut for crate::UnityEngine::UI::MaskableGraphic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl crate::UnityEngine::UI::MaskableGraphic {
    #[cfg(feature = "UnityEngine+UI+MaskableGraphic+CullStateChangedEvent")]
    pub type CullStateChangedEvent = crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent;
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
        Ok(__cordl_ret.into())
    }
    pub fn GetModifiedMaterial(
        &mut self,
        baseMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("GetModifiedMaterial", (baseMaterial))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCanvasHierarchyChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCanvasHierarchyChanged", ())?;
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
    pub fn OnTransformParentChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTransformParentChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParentMaskStateChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParentMaskStateChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateClipping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateClipping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateMasking(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateMasking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetClipRect(
        &mut self,
        clipRect: crate::UnityEngine::Rect,
        validRect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetClipRect", (clipRect, validRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetClipSoftness(
        &mut self,
        clipSoftness: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetClipSoftness", (clipSoftness))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UI_IClippable_get_gameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("UnityEngine.UI.IClippable.get_gameObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateClipParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateClipParent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCull(
        &mut self,
        cull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCull", (cull))?;
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
    pub fn get_isMaskingGraphic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMaskingGraphic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maskable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_maskable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onCullStateChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
        > = __cordl_object.invoke("get_onCullStateChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rootCanvasRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_rootCanvasRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isMaskingGraphic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isMaskingGraphic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maskable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maskable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_onCullStateChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onCullStateChanged", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::MaskableGraphic {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl AsRef<crate::UnityEngine::UI::IClippable>
for crate::UnityEngine::UI::MaskableGraphic {
    fn as_ref(&self) -> &crate::UnityEngine::UI::IClippable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl AsMut<crate::UnityEngine::UI::IClippable>
for crate::UnityEngine::UI::MaskableGraphic {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::IClippable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl AsRef<crate::UnityEngine::UI::IMaskable>
for crate::UnityEngine::UI::MaskableGraphic {
    fn as_ref(&self) -> &crate::UnityEngine::UI::IMaskable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl AsMut<crate::UnityEngine::UI::IMaskable>
for crate::UnityEngine::UI::MaskableGraphic {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::IMaskable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl AsRef<crate::UnityEngine::UI::IMaterialModifier>
for crate::UnityEngine::UI::MaskableGraphic {
    fn as_ref(&self) -> &crate::UnityEngine::UI::IMaterialModifier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
impl AsMut<crate::UnityEngine::UI::IMaterialModifier>
for crate::UnityEngine::UI::MaskableGraphic {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::IMaterialModifier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic+CullStateChangedEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct MaskableGraphic_CullStateChangedEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<bool>,
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic+CullStateChangedEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent => "UnityEngine.UI"
    ."MaskableGraphic/CullStateChangedEvent"
);
#[cfg(feature = "UnityEngine+UI+MaskableGraphic+CullStateChangedEvent")]
impl std::ops::Deref for crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<bool>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic+CullStateChangedEvent")]
impl std::ops::DerefMut
for crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic+CullStateChangedEvent")]
impl crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent {
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
#[cfg(feature = "UnityEngine+UI+MaskableGraphic+CullStateChangedEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
