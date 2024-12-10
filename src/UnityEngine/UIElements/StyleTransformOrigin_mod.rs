#[cfg(feature = "UnityEngine+UIElements+StyleTransformOrigin")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleTransformOrigin {
    pub m_Value: crate::UnityEngine::UIElements::TransformOrigin,
    pub m_Keyword: crate::UnityEngine::UIElements::StyleKeyword,
}
#[cfg(feature = "UnityEngine+UIElements+StyleTransformOrigin")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleTransformOrigin =>
    "UnityEngine.UIElements"."StyleTransformOrigin"
);
#[cfg(feature = "UnityEngine+UIElements+StyleTransformOrigin")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleTransformOrigin {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTransformOrigin")]
impl crate::UnityEngine::UIElements::StyleTransformOrigin {
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
    pub fn Equals_StyleTransformOrigin0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleTransformOrigin,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
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
    pub fn _ctor_StyleKeyword0(
        &mut self,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyword),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TransformOrigin_StyleKeyword1(
        &mut self,
        v: crate::UnityEngine::UIElements::TransformOrigin,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v, keyword),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keyword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleKeyword> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleKeyword = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_keyword",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TransformOrigin> {
        let __cordl_ret: crate::UnityEngine::UIElements::TransformOrigin = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTransformOrigin")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleTransformOrigin>,
> for crate::UnityEngine::UIElements::StyleTransformOrigin {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::StyleTransformOrigin,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTransformOrigin")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleTransformOrigin>,
> for crate::UnityEngine::UIElements::StyleTransformOrigin {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::StyleTransformOrigin,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTransformOrigin")]
impl AsRef<
    crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::TransformOrigin,
    >,
> for crate::UnityEngine::UIElements::StyleTransformOrigin {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::TransformOrigin,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleTransformOrigin")]
impl AsMut<
    crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::TransformOrigin,
    >,
> for crate::UnityEngine::UIElements::StyleTransformOrigin {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::TransformOrigin,
    > {
        todo!()
    }
}
