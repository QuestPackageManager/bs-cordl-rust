#[cfg(feature = "UnityEngine+GUISkin")]
#[repr(C)]
#[derive(Debug)]
pub struct GUISkin {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_Font: *mut crate::UnityEngine::Font,
    pub m_box: *mut crate::UnityEngine::GUIStyle,
    pub m_button: *mut crate::UnityEngine::GUIStyle,
    pub m_toggle: *mut crate::UnityEngine::GUIStyle,
    pub m_label: *mut crate::UnityEngine::GUIStyle,
    pub m_textField: *mut crate::UnityEngine::GUIStyle,
    pub m_textArea: *mut crate::UnityEngine::GUIStyle,
    pub m_window: *mut crate::UnityEngine::GUIStyle,
    pub m_horizontalSlider: *mut crate::UnityEngine::GUIStyle,
    pub m_horizontalSliderThumb: *mut crate::UnityEngine::GUIStyle,
    pub m_horizontalSliderThumbExtent: *mut crate::UnityEngine::GUIStyle,
    pub m_verticalSlider: *mut crate::UnityEngine::GUIStyle,
    pub m_verticalSliderThumb: *mut crate::UnityEngine::GUIStyle,
    pub m_verticalSliderThumbExtent: *mut crate::UnityEngine::GUIStyle,
    pub m_SliderMixed: *mut crate::UnityEngine::GUIStyle,
    pub m_horizontalScrollbar: *mut crate::UnityEngine::GUIStyle,
    pub m_horizontalScrollbarThumb: *mut crate::UnityEngine::GUIStyle,
    pub m_horizontalScrollbarLeftButton: *mut crate::UnityEngine::GUIStyle,
    pub m_horizontalScrollbarRightButton: *mut crate::UnityEngine::GUIStyle,
    pub m_verticalScrollbar: *mut crate::UnityEngine::GUIStyle,
    pub m_verticalScrollbarThumb: *mut crate::UnityEngine::GUIStyle,
    pub m_verticalScrollbarUpButton: *mut crate::UnityEngine::GUIStyle,
    pub m_verticalScrollbarDownButton: *mut crate::UnityEngine::GUIStyle,
    pub m_ScrollView: *mut crate::UnityEngine::GUIStyle,
    pub m_CustomStyles: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GUIStyle,
    >,
    pub m_Settings: *mut crate::UnityEngine::GUISettings,
    pub m_Styles: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::UnityEngine::GUIStyle,
    >,
}
#[cfg(feature = "UnityEngine+GUISkin")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUISkin => "UnityEngine"."GUISkin"
);
#[cfg(feature = "UnityEngine+GUISkin")]
impl std::ops::Deref for crate::UnityEngine::GUISkin {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUISkin")]
impl std::ops::DerefMut for crate::UnityEngine::GUISkin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUISkin")]
impl crate::UnityEngine::GUISkin {
    #[cfg(feature = "UnityEngine+GUISkin+SkinChangedDelegate")]
    pub type SkinChangedDelegate = crate::UnityEngine::GUISkin_SkinChangedDelegate;
    pub fn set_horizontalSliderThumbExtent(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalSliderThumbExtent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalScrollbar(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollbar", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_window(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_window", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_button(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_button", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_window(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_window", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_label(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_label", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalSliderThumbExtent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_horizontalSliderThumbExtent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalSlider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_horizontalSlider", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalScrollbarDownButton(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollbarDownButton", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_font(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Font> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Font = __cordl_object
            .invoke("get_font", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_textField(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textField", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalSlider(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalSlider", (value))?;
        Ok(__cordl_ret)
    }
    pub fn BuildStyleCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildStyleCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_textArea(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textArea", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sliderMixed(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sliderMixed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetStyle(
        &mut self,
        styleName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("GetStyle", (styleName))?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalScrollbarThumb(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_verticalScrollbarThumb", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_scrollView(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scrollView", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalScrollbar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_horizontalScrollbar", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalSliderThumbExtent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_verticalSliderThumbExtent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_textField", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalSliderThumb(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalSliderThumb", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalScrollbarThumb(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_horizontalScrollbarThumb", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_button(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_button", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scrollView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_scrollView", ())?;
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
    pub fn FindStyle(
        &mut self,
        styleName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("FindStyle", (styleName))?;
        Ok(__cordl_ret)
    }
    pub fn get_settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUISettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUISettings = __cordl_object
            .invoke("get_settings", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalScrollbarThumb(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollbarThumb", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_box(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_box", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderMixed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_sliderMixed", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalScrollbarRightButton(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollbarRightButton", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalSlider(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalSlider", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_textArea(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_textArea", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalSliderThumb(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_horizontalSliderThumb", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalSliderThumb(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_verticalSliderThumb", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalScrollbarUpButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_verticalScrollbarUpButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalScrollbarUpButton(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollbarUpButton", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Apply(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalSliderThumbExtent(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalSliderThumbExtent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_box(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_box", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalScrollbarThumb(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollbarThumb", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalScrollbarRightButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_horizontalScrollbarRightButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_font(
        &mut self,
        value: *mut crate::UnityEngine::Font,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_font", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalSlider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_verticalSlider", ())?;
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
    pub fn set_verticalSliderThumb(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalSliderThumb", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalScrollbar(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollbar", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_label(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_label", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalScrollbarDownButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_verticalScrollbarDownButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalScrollbarLeftButton(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollbarLeftButton", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalScrollbarLeftButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_horizontalScrollbarLeftButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn MakeCurrent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MakeCurrent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_customStyles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUIStyle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GUIStyle,
        > = __cordl_object.invoke("get_customStyles", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_customStyles(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_customStyles", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_toggle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_toggle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalScrollbar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyle = __cordl_object
            .invoke("get_verticalScrollbar", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_toggle(
        &mut self,
        value: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_toggle", (value))?;
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
#[cfg(feature = "UnityEngine+GUISkin")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUISkin {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+GUISkin+SkinChangedDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct GUISkin_SkinChangedDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+GUISkin+SkinChangedDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUISkin_SkinChangedDelegate =>
    "UnityEngine"."GUISkin/SkinChangedDelegate"
);
#[cfg(feature = "UnityEngine+GUISkin+SkinChangedDelegate")]
impl std::ops::Deref for crate::UnityEngine::GUISkin_SkinChangedDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUISkin+SkinChangedDelegate")]
impl std::ops::DerefMut for crate::UnityEngine::GUISkin_SkinChangedDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUISkin+SkinChangedDelegate")]
impl crate::UnityEngine::GUISkin_SkinChangedDelegate {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+GUISkin+SkinChangedDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::GUISkin_SkinChangedDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
