#[cfg(feature = "UnityEngine+UIElements+CreationContext")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CreationContext {
    pub _target_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _visualTreeAsset_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualTreeAsset,
    >,
    pub _slotInsertionPoints_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
    >,
    pub _attributeOverrides_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+CreationContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::CreationContext =>
    "UnityEngine.UIElements"."CreationContext"
);
#[cfg(feature = "UnityEngine+UIElements+CreationContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::CreationContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CreationContext")]
impl crate::UnityEngine::UIElements::CreationContext {
    pub fn Equals_CreationContext1(
        &mut self,
        other: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_VisualTreeAsset_VisualElement1(
        &mut self,
        slotInsertionPoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
        attributeOverrides: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
            >,
        >,
        vta: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualTreeAsset>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (slotInsertionPoints, attributeOverrides, vta, target),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_VisualTreeAsset_VisualElement0(
        &mut self,
        slotInsertionPoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
        vta: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualTreeAsset>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (slotInsertionPoints, vta, target),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_attributeOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_attributeOverrides",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_slotInsertionPoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_slotInsertionPoints",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_target(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_target", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_visualTreeAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualTreeAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualTreeAsset,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_visualTreeAsset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_attributeOverrides(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_attributeOverrides",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_slotInsertionPoints(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_slotInsertionPoints",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_target(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_target",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_visualTreeAsset(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualTreeAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_visualTreeAsset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+CreationContext")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::CreationContext>>
for crate::UnityEngine::UIElements::CreationContext {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::CreationContext> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+CreationContext")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::CreationContext>>
for crate::UnityEngine::UIElements::CreationContext {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::CreationContext,
    > {
        todo!()
    }
}
