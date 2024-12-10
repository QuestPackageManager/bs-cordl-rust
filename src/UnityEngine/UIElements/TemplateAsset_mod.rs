#[cfg(feature = "UnityEngine+UIElements+TemplateAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct TemplateAsset {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElementAsset,
    pub m_TemplateAlias: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_AttributeOverrides: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
    >,
    pub m_SlotUsages: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TemplateAsset =>
    "UnityEngine.UIElements"."TemplateAsset"
);
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TemplateAsset {
    type Target = crate::UnityEngine::UIElements::VisualElementAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TemplateAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset")]
impl crate::UnityEngine::UIElements::TemplateAsset {
    #[cfg(feature = "UnityEngine+UIElements+TemplateAsset+AttributeOverride")]
    pub type AttributeOverride = crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride;
    pub fn get_attributeOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
            >,
        > = __cordl_object.invoke("get_attributeOverrides", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_slotUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry,
            >,
        > = __cordl_object.invoke("get_slotUsages", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TemplateAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset+AttributeOverride")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TemplateAsset_AttributeOverride {
    pub m_ElementName: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_AttributeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Value: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset+AttributeOverride")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::TemplateAsset_AttributeOverride =>
    "UnityEngine.UIElements"."TemplateAsset/AttributeOverride"
);
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset+AttributeOverride")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TemplateAsset+AttributeOverride")]
impl crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride {}
