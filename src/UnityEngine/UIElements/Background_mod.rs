#[cfg(feature = "UnityEngine+UIElements+Background")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Background {
    pub m_Texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub m_Sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub m_RenderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    pub m_VectorImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VectorImage,
    >,
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
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromObject(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Background> {
        let __cordl_ret: crate::UnityEngine::UIElements::Background = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromObject", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromRenderTexture(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Background> {
        let __cordl_ret: crate::UnityEngine::UIElements::Background = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromRenderTexture", (rt))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSprite(
        s: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Background> {
        let __cordl_ret: crate::UnityEngine::UIElements::Background = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSprite", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromTexture2D(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Background> {
        let __cordl_ret: crate::UnityEngine::UIElements::Background = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromTexture2D", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromVectorImage(
        vi: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Background> {
        let __cordl_ret: crate::UnityEngine::UIElements::Background = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromVectorImage", (vi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_renderTexture",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sprite",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_texture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_texture",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vectorImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VectorImage,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_vectorImage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::Background,
        rhs: crate::UnityEngine::UIElements::Background,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::UIElements::Background,
        rhs: crate::UnityEngine::UIElements::Background,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderTexture(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_renderTexture",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sprite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_sprite",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_texture(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_texture",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vectorImage(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_vectorImage",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Background")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::Background>>
for crate::UnityEngine::UIElements::Background {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::Background> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+Background")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::Background>>
for crate::UnityEngine::UIElements::Background {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::Background> {
        todo!()
    }
}
