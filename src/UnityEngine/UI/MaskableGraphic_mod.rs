#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
#[repr(C)]
#[derive(Debug)]
pub struct MaskableGraphic {
    __cordl_parent: crate::UnityEngine::UI::Graphic,
    pub m_ShouldRecalculateStencil: bool,
    pub m_MaskMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_ParentMask: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::RectMask2D>,
    pub m_Maskable: bool,
    pub m_IsMaskingGraphic: bool,
    pub m_IncludeForMasking: bool,
    pub m_OnCullStateChanged: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
    >,
    pub m_ShouldRecalculate: bool,
    pub m_StencilValue: i32,
    pub m_Corners: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
}
#[cfg(feature = "UnityEngine+UI+MaskableGraphic")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::MaskableGraphic {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "MaskableGraphic";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Cull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "Cull", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (clipRect, validRect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetModifiedMaterial(
        &mut self,
        baseMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                1usize,
            >("GetModifiedMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "GetModifiedMaterial",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            method.invoke_unchecked(self, (baseMaterial))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("OnCanvasHierarchyChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "OnCanvasHierarchyChanged",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "OnDisable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "OnEnable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnTransformParentChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("OnTransformParentChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "OnTransformParentChanged",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParentMaskStateChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ParentMaskStateChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "ParentMaskStateChanged",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateClipping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RecalculateClipping")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "RecalculateClipping",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateMasking(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RecalculateMasking")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "RecalculateMasking", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetClipRect(
        &mut self,
        clipRect: crate::UnityEngine::Rect,
        validRect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetClipRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "SetClipRect", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (clipRect, validRect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetClipSoftness(
        &mut self,
        clipSoftness: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetClipSoftness")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "SetClipSoftness", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (clipSoftness))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UI_IClippable_get_gameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                0usize,
            >("UnityEngine.UI.IClippable.get_gameObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(),
                    "UnityEngine.UI.IClippable.get_gameObject", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateClipParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateClipParent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateClipParent", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCull(
        &mut self,
        cull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("UpdateCull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateCull", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cull))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isMaskingGraphic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isMaskingGraphic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "get_isMaskingGraphic",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maskable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_maskable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "get_maskable", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_onCullStateChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
                >,
                0usize,
            >("get_onCullStateChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "get_onCullStateChanged",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_rootCanvasRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Rect, 0usize>("get_rootCanvasRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "get_rootCanvasRect", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_isMaskingGraphic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_isMaskingGraphic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "set_isMaskingGraphic",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_maskable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_maskable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "set_maskable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_onCullStateChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_onCullStateChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic as
                    quest_hook::libil2cpp::Type > ::class(), "set_onCullStateChanged",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "MaskableGraphic/CullStateChangedEvent";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UI::MaskableGraphic_CullStateChangedEvent as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
