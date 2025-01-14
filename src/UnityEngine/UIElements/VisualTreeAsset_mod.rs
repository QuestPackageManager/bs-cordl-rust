#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeAsset {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_ImportedWithErrors: bool,
    pub m_ImportedWithWarnings: bool,
    pub m_Usings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
        >,
    >,
    pub inlineSheet: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleSheet,
    >,
    pub m_VisualElementAssets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElementAsset>,
        >,
    >,
    pub m_TemplateAssets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TemplateAsset>,
        >,
    >,
    pub m_UxmlObjectEntries: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
        >,
    >,
    pub m_UxmlObjectIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub m_AssetEntries: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry,
        >,
    >,
    pub m_Slots: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
        >,
    >,
    pub m_ContentContainerId: i32,
    pub m_ContentHash: i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::VisualTreeAsset {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualTreeAsset";
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
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualTreeAsset {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualTreeAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
impl crate::UnityEngine::UIElements::VisualTreeAsset {
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
    pub type AssetEntry = crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
    pub type SlotDefinition = crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
    pub type SlotUsageEntry = crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
    pub type UsingEntry = crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
    pub type UsingEntryComparer = crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
    pub type UxmlObjectEntry = crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry;
    pub fn AssetEntryExists(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                bool,
                2usize,
            >("AssetEntryExists")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AssetEntryExists", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (path, _cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssignClassListFromAssetToElement(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElementAsset,
        >,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElementAsset,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AssignClassListFromAssetToElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AssignClassListFromAssetToElement", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AssignStyleSheetFromAssetToElement(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElementAsset,
        >,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElementAsset,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AssignStyleSheetFromAssetToElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AssignStyleSheetFromAssetToElement", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloneSetupRecursively(
        &mut self,
        root: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElementAsset,
        >,
        idToChildren: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElementAsset,
                        >,
                    >,
                >,
            >,
        >,
        context: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElementAsset,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::UIElements::VisualElementAsset,
                                    >,
                                >,
                            >,
                        >,
                    >,
                    crate::UnityEngine::UIElements::CreationContext,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                3usize,
            >("CloneSetupRecursively")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CloneSetupRecursively", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked(self, (root, idToChildren, context)) };
        Ok(__cordl_ret.into())
    }
    pub fn CloneTree_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TemplateContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::TemplateContainer,
                >,
                0usize,
            >("CloneTree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CloneTree", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TemplateContainer,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn CloneTree_Il2CppString1(
        &mut self,
        bindingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TemplateContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::TemplateContainer,
                >,
                1usize,
            >("CloneTree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CloneTree", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TemplateContainer,
        > = unsafe { method.invoke_unchecked(self, (bindingPath)) };
        Ok(__cordl_ret.into())
    }
    pub fn CloneTree_VisualElement2(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CloneTree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CloneTree", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (target))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloneTree_VisualElement_ByRefMut_ByRefMut3(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        firstElementIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        elementAddedCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CloneTree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CloneTree", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (target, firstElementIndex, elementAddedCount))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloneTree_VisualElement_Dictionary_2_List_1_4(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        slotInsertionPoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
            >,
        >,
        attributeOverrides: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElement,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CloneTree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CloneTree", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (target, slotInsertionPoints, attributeOverrides),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareForOrder(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElementAsset>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElementAsset>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElementAsset,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElementAsset,
                    >,
                ),
                i32,
                2usize,
            >("CompareForOrder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompareForOrder", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElementAsset,
        >,
        ctx: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::VisualElementAsset,
                    >,
                    crate::UnityEngine::UIElements::CreationContext,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                2usize,
            >("Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Create", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked((), (asset, ctx)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetAsset<T>(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                T,
                1usize,
            >("GetAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAsset", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked(self, (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextChildSerialNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetNextChildSerialNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNextChildSerialNumber", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetUxmlObjectEntry(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
                1usize,
            >("GetUxmlObjectEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUxmlObjectEntry", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry = unsafe {
            method.invoke_unchecked(self, (id))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUxmlObjectFactory(
        &mut self,
        uxmlObjectAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlObjectAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IBaseUxmlObjectFactory>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UxmlObjectAsset,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IBaseUxmlObjectFactory,
                >,
                1usize,
            >("GetUxmlObjectFactory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUxmlObjectFactory", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IBaseUxmlObjectFactory,
        > = unsafe { method.invoke_unchecked(self, (uxmlObjectAsset)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetUxmlObjects<T>(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IUxmlAttributes,
        >,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::IUxmlAttributes,
                    >,
                    crate::UnityEngine::UIElements::CreationContext,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<T>,
                >,
                2usize,
            >("GetUxmlObjects")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUxmlObjects", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = unsafe { method.invoke_unchecked(self, (asset, cc)) };
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TemplateContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::TemplateContainer,
                >,
                0usize,
            >("Instantiate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Instantiate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TemplateContainer,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Il2CppString1(
        &mut self,
        bindingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TemplateContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::TemplateContainer,
                >,
                1usize,
            >("Instantiate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Instantiate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TemplateContainer,
        > = unsafe { method.invoke_unchecked(self, (bindingPath)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterAssetEntry(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RegisterAssetEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterAssetEntry", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (path, _cordl_type, asset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterUxmlObject(
        &mut self,
        uxmlObjectAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlObjectAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UxmlObjectAsset,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterUxmlObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterUxmlObject", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (uxmlObjectAsset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveTemplate(
        &mut self,
        templateName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualTreeAsset>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualTreeAsset,
                >,
                1usize,
            >("ResolveTemplate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResolveTemplate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualTreeAsset,
        > = unsafe { method.invoke_unchecked(self, (templateName)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSlotInsertionPoint(
        &mut self,
        insertionPointId: i32,
        slotName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                2usize,
            >("TryGetSlotInsertionPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetSlotInsertionPoint", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (insertionPointId, slotName))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _Create_g__CreateError_65_0(
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                1usize,
            >("<Create>g__CreateError|65_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<Create>g__CreateError|65_0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked((), (_cordl_fixed_empty_name_whitespace)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_contentContainerId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_contentContainerId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_contentContainerId", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_contentHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_contentHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_contentHash", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_importedWithErrors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_importedWithErrors")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_importedWithErrors", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_importedWithWarnings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_importedWithWarnings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_importedWithWarnings", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_slots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
                    >,
                >,
                0usize,
            >("get_slots")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_slots", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_stylesheets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::StyleSheet,
                        >,
                    >,
                >,
                0usize,
            >("get_stylesheets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_stylesheets", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_templateAssets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TemplateAsset>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::TemplateAsset,
                        >,
                    >,
                >,
                0usize,
            >("get_templateAssets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_templateAssets", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TemplateAsset>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_templateDependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualTreeAsset,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualTreeAsset,
                        >,
                    >,
                >,
                0usize,
            >("get_templateDependencies")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_templateDependencies", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualTreeAsset,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_uxmlObjectEntries(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
                    >,
                >,
                0usize,
            >("get_uxmlObjectEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_uxmlObjectEntries", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_uxmlObjectIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<i32>,
                >,
                0usize,
            >("get_uxmlObjectIds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_uxmlObjectIds", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_visualElementAssets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElementAsset,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElementAsset,
                        >,
                    >,
                >,
                0usize,
            >("get_visualElementAssets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_visualElementAssets", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElementAsset,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_contentContainerId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_contentContainerId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_contentContainerId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_contentHash(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_contentHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_contentHash", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_importedWithErrors(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_importedWithErrors")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_importedWithErrors", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_importedWithWarnings(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_importedWithWarnings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_importedWithWarnings", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_slots(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_slots")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_slots", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_templateAssets(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TemplateAsset>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::TemplateAsset,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_templateAssets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_templateAssets", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_visualElementAssets(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElementAsset,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElementAsset,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_visualElementAssets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_visualElementAssets", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisualTreeAsset_AssetEntry {
    pub path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub typeFullName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    pub m_CachedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualTreeAsset/AssetEntry";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    pub fn _ctor(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (path, _cordl_type, asset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                0usize,
            >("get_type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_type", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisualTreeAsset_SlotDefinition {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub insertionPointId: i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualTreeAsset/SlotDefinition";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisualTreeAsset_SlotUsageEntry {
    pub slotName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub assetId: i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualTreeAsset/SlotUsageEntry";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisualTreeAsset_UsingEntry {
    pub alias: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub asset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualTreeAsset,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualTreeAsset/UsingEntry";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    pub fn _ctor(
        &mut self,
        alias: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alias, path))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeAsset_UsingEntryComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualTreeAsset/UsingEntryComparer";
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
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    pub fn Compare(
        &mut self,
        x: crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
        y: crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
                    crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
                ),
                i32,
                2usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Compare", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (x, y)) };
        Ok(__cordl_ret.into())
    }
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl AsRef<
    crate::System::Collections::Generic::IComparer_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
    >,
> for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl AsMut<
    crate::System::Collections::Generic::IComparer_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
    >,
> for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisualTreeAsset_UxmlObjectEntry {
    pub parentId: i32,
    pub uxmlObjectAssets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UxmlObjectAsset>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualTreeAsset/UxmlObjectEntry";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    pub fn _ctor(
        &mut self,
        parentId: i32,
        uxmlObjectAssets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UxmlObjectAsset,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::UxmlObjectAsset,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parentId, uxmlObjectAssets))
        };
        Ok(__cordl_ret.into())
    }
}
