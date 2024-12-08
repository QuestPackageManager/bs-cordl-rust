#[cfg(feature = "UnityEngine+UIElements+Background")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Background {
    pub m_Texture: *mut crate::UnityEngine::Texture2D,
    pub m_Sprite: *mut crate::UnityEngine::Sprite,
    pub m_RenderTexture: *mut crate::UnityEngine::RenderTexture,
    pub m_VectorImage: *mut crate::UnityEngine::UIElements::VectorImage,
}
#[cfg(feature = "UnityEngine+UIElements+Background")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Background =>
    "UnityEngine.UIElements"."Background"
);
#[cfg(feature = "UnityEngine+UIElements+Background")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Background {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Background")]
impl crate::UnityEngine::UIElements::Background {
    pub fn Equals_Background0(
        &mut self,
        other: crate::UnityEngine::UIElements::Background,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_renderTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RenderTexture> {
        let __cordl_ret: *mut crate::UnityEngine::RenderTexture = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_renderTexture",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_sprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_ret: *mut crate::UnityEngine::Sprite = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sprite",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_texture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_texture",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_vectorImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VectorImage,
    > {
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VectorImage = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_vectorImage",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_renderTexture(
        &mut self,
        value: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_renderTexture",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_sprite(
        &mut self,
        value: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_sprite",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_texture(
        &mut self,
        value: *mut crate::UnityEngine::Texture2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_texture",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_vectorImage(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::VectorImage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_vectorImage",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
