#[cfg(feature = "UnityEngine+UIElements+Experimental+StyleValues")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleValues {
    pub m_StyleValues: *mut crate::UnityEngine::UIElements::StyleValueCollection,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+StyleValues")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Experimental::StyleValues =>
    "UnityEngine.UIElements.Experimental"."StyleValues"
);
#[cfg(feature = "UnityEngine+UIElements+Experimental+StyleValues")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Experimental::StyleValues {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+StyleValues")]
impl crate::UnityEngine::UIElements::Experimental::StyleValues {
    pub fn SetValue_Color1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetValue",
            (id, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetValue_f32_0(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetValue",
            (id, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleValueCollection,
    > {
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleValueCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Values",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_paddingTop(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_paddingTop",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_backgroundColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_backgroundColor",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderBottomLeftRadius(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderBottomLeftRadius",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderBottomRightRadius(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderBottomRightRadius",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderBottomWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderBottomWidth",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderColor",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderLeftWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderLeftWidth",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderRightWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderRightWidth",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderTopLeftRadius(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderTopLeftRadius",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderTopRightRadius(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderTopRightRadius",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_borderTopWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_borderTopWidth",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_bottom(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bottom",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_color",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_flexGrow(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_flexGrow",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_flexShrink(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_flexShrink",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_height(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_height",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_left(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_left",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_marginBottom(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_marginBottom",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_marginLeft(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_marginLeft",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_marginRight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_marginRight",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_marginTop(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_marginTop",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_opacity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_opacity",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_paddingBottom(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_paddingBottom",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_paddingLeft(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_paddingLeft",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_paddingRight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_paddingRight",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_paddingTop(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_paddingTop",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_right(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_right",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_top(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_top",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_unityBackgroundImageTintColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_unityBackgroundImageTintColor",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_width(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_width",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
