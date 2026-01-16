#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputDeviceBuilder {
    pub m_Device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    pub m_LayoutCacheRef:
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance,
    pub m_ChildControlOverrides: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    >,
    pub m_StateOffsetToControlMap:
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<u32>>,
    pub m_StringBuilder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputDeviceBuilder";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
impl crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder {
    pub const kSizeForControlUsingStateFromOtherControl: u32 = 4294967295u32;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
    pub type RefInstance = crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance;
    pub fn AddChildControl(
        &mut self,
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        haveChildrenUsingStateFromOtherControls: quest_hook::libil2cpp::ByRefMut<bool>,
        controlItem: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        childIndex: i32,
        nameOverride: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                            >,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                        7usize,
                    >("AddChildControl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddChildControl", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    layout,
                    variants,
                    parent,
                    haveChildrenUsingStateFromOtherControls,
                    controlItem,
                    childIndex,
                    nameOverride,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddChildControlIfMissing(
        &mut self,
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        haveChildrenUsingStateFromOtherControls: quest_hook::libil2cpp::ByRefMut<bool>,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                            >,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("AddChildControlIfMissing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddChildControlIfMissing", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    layout,
                    variants,
                    parent,
                    haveChildrenUsingStateFromOtherControls,
                    controlItem,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddChildControls(
        &mut self,
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        haveChildrenUsingStateFromOtherControls: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                        >,
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        quest_hook::libil2cpp::ByRefMut<bool>,
                    ), quest_hook::libil2cpp::Void, 4usize>("AddChildControls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddChildControls",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    layout,
                    variants,
                    parent,
                    haveChildrenUsingStateFromOtherControls,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddChildren(
        &mut self,
        parent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        >,
        left: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        right: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
                        >,
                        crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
                        crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
                    ), quest_hook::libil2cpp::Void, 3usize>("AddChildren")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddChildren",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parent, left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddControlToNode(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controlIndiciesNextFreeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        nodeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("AddControlToNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddControlToNode",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (control, controlIndiciesNextFreeIndex, nodeIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddParentDisplayNameRecursive(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        stringBuilder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        shortName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "AddParentDisplayNameRecursive"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddParentDisplayNameRecursive",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (control, stringBuilder, shortName))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddProcessors(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddProcessors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddProcessors", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (control, controlItem, layoutName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyUseStateFrom(
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ApplyUseStateFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyUseStateFrom", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (parent, controlItem, layout))? };
        Ok(__cordl_ret.into())
    }
    pub fn ChildControlOverridePath(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controlName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, 2usize>(
                        "ChildControlOverridePath",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ChildControlOverridePath",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, (parent, controlName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeStateLayout(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ComputeStateLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeStateLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (control))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeControlHierarchy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "FinalizeControlHierarchy",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FinalizeControlHierarchy",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeControlHierarchyRecursive(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controlIndex: i32,
        allControls: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        >,
        noisy: bool,
        dontReset: bool,
        controlIndiciesNextFreeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::InputSystem::InputControl,
                                >,
                            >,
                        >,
                        bool,
                        bool,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "FinalizeControlHierarchyRecursive"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FinalizeControlHierarchyRecursive",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    control,
                    controlIndex,
                    allControls,
                    noisy,
                    dontReset,
                    controlIndiciesNextFreeIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindOrLoadLayout(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Layouts::InputControlLayout>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                        >,
                        1usize,
                    >("FindOrLoadLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindOrLoadLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = unsafe { cordl_method_info.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >,
                        0usize,
                    >("Finish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Finish",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBestMidPoint(
        &mut self,
        parent: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        startOffset: u16,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
                        u16,
                    ), u16, 2usize>("GetBestMidPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetBestMidPoint",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u16 =
            unsafe { cordl_method_info.invoke_unchecked(self, (parent, startOffset))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetControlIndex(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        u16,
                        1usize,
                    >("GetControlIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetControlIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe { cordl_method_info.invoke_unchecked(self, (control))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertChildControl(
        &mut self,
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
        variant: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        haveChildrenUsingStateFromOtherControls: quest_hook::libil2cpp::ByRefMut<bool>,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                            >,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                        5usize,
                    >("InsertChildControl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertChildControl", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    layout,
                    variant,
                    parent,
                    haveChildrenUsingStateFromOtherControls,
                    controlItem,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertChildControlOverride(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InsertChildControlOverride")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertChildControlOverride", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parent, controlItem))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertControlBitRangeNode(
        &mut self,
        parent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        >,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controlIndiciesNextFreeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        startOffset: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        u16,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "InsertControlBitRangeNode"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InsertControlBitRangeNode",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (parent, control, controlIndiciesNextFreeIndex, startOffset),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateLayout_InputControlLayout1(
        &mut self,
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
                            >,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                        4usize,
                    >("InstantiateLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstantiateLayout", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl> =
            unsafe { cordl_method_info.invoke_unchecked(self, (layout, variants, name, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateLayout_InternedString0(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                        4usize,
                    >("InstantiateLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstantiateLayout", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl> =
            unsafe { cordl_method_info.invoke_unchecked(self, (layout, variants, name, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn Ref() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance,
                        0usize,
                    >("Ref")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Ref",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetDisplayName(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        longDisplayNameFromLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shortDisplayNameFromLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shortName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDisplayName",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    control,
                    longDisplayNameFromLayout,
                    shortDisplayNameFromLayout,
                    shortName,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFormat(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controlItem: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetFormat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (control, controlItem))? };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        crate::UnityEngine::InputSystem::Utilities::InternedString,
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
                    ), quest_hook::libil2cpp::Void, 3usize>("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Setup",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (layout, variants, deviceDescription))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftChildIndicesInHierarchyOneUp(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        startIndex: i32,
        exceptControl: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ShiftChildIndicesInHierarchyOneUp"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShiftChildIndicesInHierarchyOneUp",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (device, startIndex, exceptControl))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder,
                    >, 0usize>("get_instance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_instance",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputDeviceBuilder_RefInstance {}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Layouts";
    const CLASS_NAME: &'static str = "InputDeviceBuilder/RefInstance";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
impl crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance {
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
