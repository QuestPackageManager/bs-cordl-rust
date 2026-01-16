#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+InspectorPanel")]
#[repr(C)]
#[derive(Debug)]
pub struct InspectorPanel {
    __cordl_parent: crate::Meta::XR::ImmersiveDebugger::UserInterface::DebugPanel,
    pub _scrollView: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ScrollView,
    >,
    pub _categoryScrollView: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ScrollView,
    >,
    pub _hierarchyScrollView: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ScrollView,
    >,
    pub _registries: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::Dictionary_2<
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::UserInterface::Inspector,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
    pub _categories: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
            quest_hook::libil2cpp::Gc<
                crate::Meta::XR::ImmersiveDebugger::UserInterface::CategoryButton,
            >,
        >,
    >,
    pub _items: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<
                crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
            >,
            quest_hook::libil2cpp::Gc<
                crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
            >,
        >,
    >,
    pub _selectedCategory: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::CategoryButton,
    >,
    pub _selectedItem: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
    >,
    pub _categoryBackground: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Background,
    >,
    pub _currentPosition: crate::UnityEngine::Vector3,
    pub _targetPosition: crate::UnityEngine::Vector3,
    pub _lerpSpeed: f32,
    pub _lerpCompleted: bool,
    pub _categoryBackgroundImageStyle: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ImageStyle,
    >,
    pub _debugInterface: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::DebugInterface,
    >,
    pub _buttonsAnchor: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
    >,
    pub _selectedModeTitle: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Label,
    >,
    pub _hierarchyIcon: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Toggle,
    >,
    pub _categoriesIcon: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Toggle,
    >,
    pub _categoryDiv: quest_hook::libil2cpp::Gc<
        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
    >,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+InspectorPanel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::ImmersiveDebugger::UserInterface::InspectorPanel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.UserInterface";
    const CLASS_NAME: &'static str = "InspectorPanel";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+InspectorPanel")]
impl std::ops::Deref
for crate::Meta::XR::ImmersiveDebugger::UserInterface::InspectorPanel {
    type Target = crate::Meta::XR::ImmersiveDebugger::UserInterface::DebugPanel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+InspectorPanel")]
impl std::ops::DerefMut
for crate::Meta::XR::ImmersiveDebugger::UserInterface::InspectorPanel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+InspectorPanel")]
impl crate::Meta::XR::ImmersiveDebugger::UserInterface::InspectorPanel {
    pub fn ComputeIdealPreviousItem(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Controller,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Controller,
                        >,
                        1usize,
                    >("ComputeIdealPreviousItem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeIdealPreviousItem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Controller,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (item))? };
        Ok(__cordl_ret.into())
    }
    pub fn FoldItem(
        &mut self,
        button: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FoldItem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FoldItem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (button))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCategoryButton(
        &mut self,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
        create: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::CategoryButton,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Meta::XR::ImmersiveDebugger::Manager::Category, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::CategoryButton,
                        >,
                        2usize,
                    >("GetCategoryButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCategoryButton", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::CategoryButton,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (category, create))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHierarchyItemButton(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
        >,
        create: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
                        >,
                        2usize,
                    >("GetHierarchyItemButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHierarchyItemButton", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (item, create))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetInspector(
        &mut self,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
                        >,
                        2usize,
                    >("GetInspector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInspector", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (instanceHandle, category))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInspectorInternal(
        &mut self,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
        createRegistries: bool,
        registry: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                    quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::ImmersiveDebugger::UserInterface::Inspector,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Inspector,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::Dictionary_2<
                                        crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                                        quest_hook::libil2cpp::Gc<
                                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Inspector,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Inspector,
                        >,
                        4usize,
                    >("GetInspectorInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInspectorInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Inspector,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (instanceHandle, category, createRegistries, registry),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnTransparencyChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("OnTransparencyChanged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnTransparencyChanged", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterControl(
        &mut self,
        buttonName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        icon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        style: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ImageStyle,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Toggle,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ImageStyle,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Toggle,
                        >,
                        4usize,
                    >("RegisterControl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterControl", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Toggle,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (buttonName, icon, style, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInspector(
        &mut self,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
                        >,
                        2usize,
                    >("RegisterInspector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterInspector", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IInspector,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (instanceHandle, category))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveInspector(
        &mut self,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
        inspector: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Inspector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::UserInterface::Inspector,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RemoveInspector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveInspector", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (category, inspector))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectCategory(
        &mut self,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Meta::XR::ImmersiveDebugger::Manager::Category),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SelectCategory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectCategory", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (category))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectCategoryButton(
        &mut self,
        categoryButton: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::CategoryButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::CategoryButton,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SelectCategoryButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectCategoryButton", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (categoryButton))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectCategoryMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("SelectCategoryMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectCategoryMode", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectHierarchyItemButton(
        &mut self,
        button: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SelectHierarchyItemButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectHierarchyItemButton", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (button))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectHierarchyMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("SelectHierarchyMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectHierarchyMode", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectItem(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SelectItem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectItem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (item))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPanelPosition(
        &mut self,
        distanceOption: crate::Meta::XR::ImmersiveDebugger::RuntimeSettings_DistanceOption,
        skipAnimation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::RuntimeSettings_DistanceOption,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetPanelPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetPanelPosition", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (distanceOption, skipAnimation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Controller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Controller,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Setup",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (owner))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToggleFoldItem(
        &mut self,
        button: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ToggleFoldItem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToggleFoldItem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (button))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryRemoveHierarchyItemButton(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Hierarchy::Item,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("TryRemoveHierarchyItemButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryRemoveHierarchyItemButton", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (item))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnfoldItem(
        &mut self,
        button: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::HierarchyItemButton,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnfoldItem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnfoldItem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (button))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterInspector(
        &mut self,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        category: crate::Meta::XR::ImmersiveDebugger::Manager::Category,
        allCategories: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            crate::Meta::XR::ImmersiveDebugger::Manager::Category,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("UnregisterInspector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnregisterInspector", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (instanceHandle, category, allCategories))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Update",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoryFlex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
                        >,
                        0usize,
                    >("get_CategoryFlex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CategoryFlex", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Flex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
                        >,
                        0usize,
                    >("get_Flex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Flex", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_HierarchyFlex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
                        >,
                        0usize,
                    >("get_HierarchyFlex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_HierarchyFlex", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Flex,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ScrollView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ScrollView,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ScrollView,
                        >,
                        0usize,
                    >("get_ScrollView")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ScrollView", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ScrollView,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_CategoryBackgroundStyle(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ImageStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::ImageStyle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_CategoryBackgroundStyle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_CategoryBackgroundStyle", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+InspectorPanel")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::ImmersiveDebugger::UserInterface::InspectorPanel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+InspectorPanel")]
impl AsRef<crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel>
for crate::Meta::XR::ImmersiveDebugger::UserInterface::InspectorPanel {
    fn as_ref(
        &self,
    ) -> &crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+InspectorPanel")]
impl AsMut<crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel>
for crate::Meta::XR::ImmersiveDebugger::UserInterface::InspectorPanel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel {
        unsafe { std::mem::transmute(self) }
    }
}
