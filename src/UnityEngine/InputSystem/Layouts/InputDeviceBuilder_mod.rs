#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputDeviceBuilder {
    pub m_Device: *mut crate::UnityEngine::InputSystem::InputDevice,
    pub m_LayoutCacheRef: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance,
    pub m_ChildControlOverrides: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    >,
    pub m_StateOffsetToControlMap: *mut crate::System::Collections::Generic::List_1<u32>,
    pub m_StringBuilder: *mut crate::System::Text::StringBuilder,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputDeviceBuilder =>
    "UnityEngine.InputSystem.Layouts"."InputDeviceBuilder"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder {
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
    pub const kSizeForControlUsingStateFromOtherControl: u32 = 16843263u32;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
    pub type RefInstance = crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance;
    pub fn AddChildControl(
        &mut self,
        layout: *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
        haveChildrenUsingStateFromOtherControls: quest_hook::libil2cpp::ByRefMut<bool>,
        controlItem: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        childIndex: i32,
        nameOverride: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputControl,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputControl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddChildControl",
            (
                layout,
                variants,
                parent,
                haveChildrenUsingStateFromOtherControls,
                controlItem,
                childIndex,
                nameOverride,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddChildControlIfMissing(
        &mut self,
        layout: *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
        haveChildrenUsingStateFromOtherControls: quest_hook::libil2cpp::ByRefMut<bool>,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddChildControlIfMissing",
            (
                layout,
                variants,
                parent,
                haveChildrenUsingStateFromOtherControls,
                controlItem,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddChildControls(
        &mut self,
        layout: *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
        haveChildrenUsingStateFromOtherControls: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddChildControls",
            (layout, variants, parent, haveChildrenUsingStateFromOtherControls),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddChildren(
        &mut self,
        parent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        >,
        left: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        right: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddChildren",
            (parent, left, right),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddControlToNode(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        controlIndiciesNextFreeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        nodeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddControlToNode",
            (control, controlIndiciesNextFreeIndex, nodeIndex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ChildControlOverridePath(
        &mut self,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
        controlName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ChildControlOverridePath",
            (parent, controlName),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FinalizeControlHierarchy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FinalizeControlHierarchy",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FinalizeControlHierarchyRecursive(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        controlIndex: i32,
        allControls: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
        noisy: bool,
        dontReset: bool,
        controlIndiciesNextFreeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FinalizeControlHierarchyRecursive",
            (
                control,
                controlIndex,
                allControls,
                noisy,
                dontReset,
                controlIndiciesNextFreeIndex,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Finish",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetBestMidPoint(
        &mut self,
        parent: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        startOffset: u16,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBestMidPoint",
            (parent, startOffset),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetControlIndex(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetControlIndex",
            (control),
        )?;
        Ok(__cordl_ret)
    }
    pub fn InsertChildControl(
        &mut self,
        layout: *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        variant: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
        haveChildrenUsingStateFromOtherControls: quest_hook::libil2cpp::ByRefMut<bool>,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputControl,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputControl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InsertChildControl",
            (
                layout,
                variant,
                parent,
                haveChildrenUsingStateFromOtherControls,
                controlItem,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn InsertChildControlOverride(
        &mut self,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InsertChildControlOverride",
            (parent, controlItem),
        )?;
        Ok(__cordl_ret)
    }
    pub fn InsertControlBitRangeNode(
        &mut self,
        parent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        >,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        controlIndiciesNextFreeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        startOffset: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InsertControlBitRangeNode",
            (parent, control, controlIndiciesNextFreeIndex, startOffset),
        )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateLayout_InputControlLayout1(
        &mut self,
        layout: *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputControl,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputControl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InstantiateLayout",
            (layout, variants, name, parent),
        )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateLayout_InternedString0(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputControl,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputControl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InstantiateLayout",
            (layout, variants, name, parent),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetDisplayName(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        longDisplayNameFromLayout: *mut crate::System::String,
        shortDisplayNameFromLayout: *mut crate::System::String,
        shortName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetDisplayName",
            (control, longDisplayNameFromLayout, shortDisplayNameFromLayout, shortName),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Setup",
            (layout, variants, deviceDescription),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputDeviceBuilder_RefInstance {}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance =>
    "UnityEngine.InputSystem.Layouts"."InputDeviceBuilder/RefInstance"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceBuilder+RefInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputDeviceBuilder_RefInstance {
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
