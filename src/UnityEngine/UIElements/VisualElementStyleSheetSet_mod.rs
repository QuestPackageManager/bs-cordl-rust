#[cfg(feature = "UnityEngine+UIElements+VisualElementStyleSheetSet")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct VisualElementStyleSheetSet {
    pub m_Element: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementStyleSheetSet")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElementStyleSheetSet => "UnityEngine.UIElements"
    ."VisualElementStyleSheetSet"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementStyleSheetSet")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualElementStyleSheetSet {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementStyleSheetSet")]
impl crate::UnityEngine::UIElements::VisualElementStyleSheetSet {
    pub fn Add(
        &mut self,
        styleSheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (styleSheet),
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
    pub fn Equals_VisualElementStyleSheetSet0(
        &mut self,
        other: crate::UnityEngine::UIElements::VisualElementStyleSheetSet,
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
    pub fn Remove(
        &mut self,
        styleSheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (styleSheet),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (element),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementStyleSheetSet")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::VisualElementStyleSheetSet,
    >,
> for crate::UnityEngine::UIElements::VisualElementStyleSheetSet {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::VisualElementStyleSheetSet,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementStyleSheetSet")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::VisualElementStyleSheetSet,
    >,
> for crate::UnityEngine::UIElements::VisualElementStyleSheetSet {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::VisualElementStyleSheetSet,
    > {
        todo!()
    }
}
