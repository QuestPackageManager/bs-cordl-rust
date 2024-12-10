#[cfg(feature = "UnityEngine+UI+RectMask2D")]
#[repr(C)]
#[derive(Debug)]
pub struct RectMask2D {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_VertexClipper: *mut crate::UnityEngine::UI::RectangularVertexClipper,
    pub m_RectTransform: *mut crate::UnityEngine::RectTransform,
    pub m_MaskableTargets: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UI::MaskableGraphic,
    >,
    pub m_ClipTargets: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UI::IClippable,
    >,
    pub m_ShouldRecalculateClipRects: bool,
    pub m_Clippers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UI::RectMask2D,
    >,
    pub m_LastClipRectCanvasSpace: crate::UnityEngine::Rect,
    pub m_ForceClip: bool,
    pub m_Padding: crate::UnityEngine::Vector4,
    pub m_Softness: crate::UnityEngine::Vector2Int,
    pub m_Canvas: *mut crate::UnityEngine::Canvas,
    pub m_Corners: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::RectMask2D => "UnityEngine.UI"
    ."RectMask2D"
);
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
impl std::ops::Deref for crate::UnityEngine::UI::RectMask2D {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
impl std::ops::DerefMut for crate::UnityEngine::UI::RectMask2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
impl crate::UnityEngine::UI::RectMask2D {
    pub fn AddClippable(
        &mut self,
        clippable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::IClippable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddClippable", (clippable))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRaycastLocationValid(
        &mut self,
        sp: crate::UnityEngine::Vector2,
        eventCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsRaycastLocationValid", (sp, eventCamera))?;
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
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
    pub fn PerformClipping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformClipping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveClippable(
        &mut self,
        clippable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::IClippable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveClippable", (clippable))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateClipSoftness(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateClipSoftness", ())?;
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
    pub fn get_Canvas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas> = __cordl_object
            .invoke("get_Canvas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canvasRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_canvasRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_padding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("get_padding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_rectTransform", ())?;
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
    pub fn get_softness(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2Int = __cordl_object
            .invoke("get_softness", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_padding(
        &mut self,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_padding", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_softness(
        &mut self,
        value: crate::UnityEngine::Vector2Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_softness", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::RectMask2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
impl AsRef<crate::UnityEngine::ICanvasRaycastFilter>
for crate::UnityEngine::UI::RectMask2D {
    fn as_ref(&self) -> &crate::UnityEngine::ICanvasRaycastFilter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
impl AsMut<crate::UnityEngine::ICanvasRaycastFilter>
for crate::UnityEngine::UI::RectMask2D {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ICanvasRaycastFilter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
impl AsRef<crate::UnityEngine::UI::IClipper> for crate::UnityEngine::UI::RectMask2D {
    fn as_ref(&self) -> &crate::UnityEngine::UI::IClipper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+RectMask2D")]
impl AsMut<crate::UnityEngine::UI::IClipper> for crate::UnityEngine::UI::RectMask2D {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::IClipper {
        unsafe { std::mem::transmute(self) }
    }
}
