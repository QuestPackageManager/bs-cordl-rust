#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Builder_InputControlLayout_ControlBuilder {
    pub builder: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
    >,
    pub index: i32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/Builder/ControlBuilder"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
)]
impl crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder {
    pub fn AsArrayOfControlsWithSize(
        &mut self,
        arraySize: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AsArrayOfControlsWithSize",
            (arraySize),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DontReset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DontReset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNoisy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSynthetic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsSynthetic",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UsingStateFrom(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UsingStateFrom",
            (path),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithBitOffset(
        &mut self,
        bit: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithBitOffset",
            (bit),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithByteOffset(
        &mut self,
        offset: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithByteOffset",
            (offset),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithDefaultState(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithDefaultState",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithDisplayName(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithDisplayName",
            (displayName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFormat_FourCC0(
        &mut self,
        format: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithFormat",
            (format),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFormat_Il2CppString1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithFormat",
            (format),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithLayout(
        &mut self,
        layout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithLayout",
            (layout),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithParameters(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithParameters",
            (parameters),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithProcessors(
        &mut self,
        processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithProcessors",
            (processors),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithRange(
        &mut self,
        minValue: f32,
        maxValue: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithRange",
            (minValue, maxValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithSizeInBits(
        &mut self,
        sizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithSizeInBits",
            (sizeInBits),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithUsages_IEnumerable_1_1(
        &mut self,
        usages: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithUsages",
            (usages),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithUsages_Il2CppArray0(
        &mut self,
        usages: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithUsages",
            (usages),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithUsages_Il2CppArray2(
        &mut self,
        usages: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithUsages",
            (usages),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Collection_InputControlLayout_LayoutMatcher {
    pub layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub deviceMatcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/Collection/LayoutMatcher"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
)]
impl crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher {}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Collection_InputControlLayout_PrecompiledLayout {
    pub factoryMethod: quest_hook::libil2cpp::Gc<
        crate::System::Func_1<*mut crate::UnityEngine::InputSystem::InputDevice>,
    >,
    pub metadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout
    => "UnityEngine.InputSystem.Layouts"
    ."InputControlLayout/Collection/PrecompiledLayout"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
)]
impl crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout {}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlItem_InputControlLayout_Flags {
    DontReset = 16i32,
    IsFirstDefinedInThisLayout = 8i32,
    IsNoisy = 2i32,
    IsSynthetic = 4i32,
    isModifyingExistingControl = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/ControlItem/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlLayout {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Name: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub m_Type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_Variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub m_StateFormat: crate::UnityEngine::InputSystem::Utilities::FourCC,
    pub m_StateSizeInBytes: i32,
    pub m_UpdateBeforeRender: crate::System::Nullable_1<bool>,
    pub m_BaseLayouts: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub m_AppliedOverrides: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub m_CommonUsages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    >,
    pub m_Controls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    >,
    pub m_DisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Flags: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Layouts::InputControlLayout {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout {
    pub const VariantSeparator: &'static str = ";";
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
    pub type Builder = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
    pub type Cache = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance"
    )]
    pub type CacheRefInstance = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
    pub type Collection = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
    pub type ControlItem = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson"
    )]
    pub type ControlItemJson = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags;
    #[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
    pub type LayoutJson = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
    )]
    pub type LayoutJsonNameAndDescriptorOnly = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
    )]
    pub type LayoutNotFoundException = crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException;
    pub fn AddControlItems(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        controlLayouts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddControlItems", (_cordl_type, controlLayouts, layoutName))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddControlItemsFromFields(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        controlLayouts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddControlItemsFromFields",
                (_cordl_type, controlLayouts, layoutName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddControlItemsFromMember(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::InputSystem::Layouts::InputControlAttribute,
            >,
        >,
        controlItems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddControlItemsFromMember", (member, attributes, controlItems))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddControlItemsFromMembers(
        members: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MemberInfo,
            >,
        >,
        controlItems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddControlItemsFromMembers", (members, controlItems, layoutName))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddControlItemsFromProperties(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        controlLayouts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddControlItemsFromProperties",
                (_cordl_type, controlLayouts, layoutName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CacheRef() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CacheRef", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateControlItemFromMember(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attribute: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateControlItemFromMember", (member, attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLookupTableForControls(
        controlItems: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
        variants: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateLookupTableForControls", (controlItems, variants))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControl(
        &mut self,
        path: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        > = __cordl_object.invoke("FindControl", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControlIncludingArrayElements(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arrayIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        > = __cordl_object
            .invoke("FindControlIncludingArrayElements", (path, arrayIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromJson(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromJson", (json))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromType(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromType", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetValueType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InferLayoutFromValueType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InferLayoutFromValueType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn MergeLayout(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeLayout", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseHeaderFieldsFromJson(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
        baseLayouts: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
        deviceMatcher: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ParseHeaderFieldsFromJson",
                (json, name, baseLayouts, deviceMatcher),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToJson", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VariantsMatch_Il2CppString_Il2CppString1(
        expected: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        actual: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VariantsMatch", (expected, actual))?;
        Ok(__cordl_ret.into())
    }
    pub fn VariantsMatch_InternedString_InternedString0(
        expected: crate::UnityEngine::InputSystem::Utilities::InternedString,
        actual: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VariantsMatch", (expected, actual))?;
        Ok(__cordl_ret.into())
    }
    pub fn _MergeLayout_b__77_0(
        &mut self,
        x: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("<MergeLayout>b__77_0", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultVariant() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultVariant", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem = __cordl_object
            .invoke("get_Item", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_appliedOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        > = __cordl_object.invoke("get_appliedOverrides", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_baseLayouts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        > = __cordl_object.invoke("get_baseLayouts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cache() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_cache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canRunInBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("get_canRunInBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_commonUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        > = __cordl_object.invoke("get_commonUsages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        > = __cordl_object.invoke("get_controls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_displayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_displayName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hideInUI(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hideInUI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isControlLayout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isControlLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDeviceLayout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDeviceLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isGenericTypeOfDevice(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isGenericTypeOfDevice", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isNoisy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isNoisy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isOverride(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isOverride", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = __cordl_object
            .invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stateFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = __cordl_object
            .invoke("get_stateFormat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_stateSizeInBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateBeforeRender(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_updateBeforeRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_variants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = __cordl_object
            .invoke("get_variants", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_canRunInBackground(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_canRunInBackground", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hideInUI(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hideInUI", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isGenericTypeOfDevice(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isGenericTypeOfDevice", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isNoisy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isOverride(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isOverride", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlLayout_Builder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _displayName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _type_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _stateFormat_k__BackingField: crate::UnityEngine::InputSystem::Utilities::FourCC,
    pub _stateSizeInBytes_k__BackingField: i32,
    pub m_ExtendsLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _updateBeforeRender_k__BackingField: crate::System::Nullable_1<bool>,
    pub m_ControlCount: i32,
    pub m_Controls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/Builder"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder {
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder+ControlBuilder"
    )]
    pub type ControlBuilder = crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder;
    pub fn AddControl(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::Builder_InputControlLayout_ControlBuilder = __cordl_object
            .invoke("AddControl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = __cordl_object.invoke("Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Extend(
        &mut self,
        baseLayoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = __cordl_object.invoke("Extend", (baseLayoutName))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn WithDisplayName(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = __cordl_object.invoke("WithDisplayName", (displayName))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFormat_FourCC0(
        &mut self,
        format: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = __cordl_object.invoke("WithFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithFormat_Il2CppString1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = __cordl_object.invoke("WithFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = __cordl_object.invoke("WithName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithSizeInBytes(
        &mut self,
        sizeInBytes: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = __cordl_object.invoke("WithSizeInBytes", (sizeInBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithType<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
        > = __cordl_object.invoke("WithType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        > = __cordl_object.invoke("get_controls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_displayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_displayName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_extendsLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_extendsLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stateFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = __cordl_object
            .invoke("get_stateFormat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_stateSizeInBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateBeforeRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("get_updateBeforeRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_displayName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_extendsLayout(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_extendsLayout", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stateFormat(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stateFormat", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stateSizeInBytes(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stateSizeInBytes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_type(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_type", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_updateBeforeRender(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_updateBeforeRender", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Builder")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputControlLayout_Cache {
    pub table: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/Cache"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Cache")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindOrLoadLayout(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        throwIfNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FindOrLoadLayout",
            (name, throwIfNotFound),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputControlLayout_CacheRefInstance {
    pub valid: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/CacheRefInstance"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+CacheRefInstance")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_CacheRefInstance {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputControlLayout_Collection {
    pub layoutTypes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            *mut crate::System::Type,
        >,
    >,
    pub layoutStrings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub layoutBuilders: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            *mut crate::System::Func_1<
                *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
            >,
        >,
    >,
    pub baseLayoutTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    >,
    pub layoutOverrides: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
    >,
    pub layoutOverrideNames: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    >,
    pub precompiledLayouts: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
            crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout,
        >,
    >,
    pub layoutMatchers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/Collection"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection {
    pub const kBaseScoreForNonGeneratedLayouts: f32 = 1f32;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+LayoutMatcher"
    )]
    pub type LayoutMatcher = crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_LayoutMatcher;
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Collection+PrecompiledLayout"
    )]
    pub type PrecompiledLayout = crate::UnityEngine::InputSystem::Layouts::Collection_InputControlLayout_PrecompiledLayout;
    pub fn AddMatcher(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddMatcher",
            (layout, matcher),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Allocate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Allocate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeDistanceInInheritanceHierarchy(
        &mut self,
        firstLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        secondLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        distance: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ComputeDistanceInInheritanceHierarchy",
            (firstLayout, secondLayout, distance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindLayoutThatIntroducesControl(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        cache: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FindLayoutThatIntroducesControl",
            (control, cache),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseLayoutName(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBaseLayoutName",
            (layoutName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseLayouts(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        includeSelf: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBaseLayouts",
            (layout, includeSelf),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControlTypeForLayout(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetControlTypeForLayout",
            (layoutName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootLayoutName(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRootLayoutName",
            (layoutName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLayout(
        &mut self,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasLayout",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBasedOn(
        &mut self,
        parentLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        childLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsBasedOn",
            (parentLayout, childLayout),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGeneratedLayout(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsGeneratedLayout",
            (layout),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindLayoutForType(
        &mut self,
        layoutType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFindLayoutForType",
            (layoutType),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindMatchingLayout(
        &mut self,
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFindMatchingLayout",
            (deviceDescription),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryLoadLayout(
        &mut self,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
        table: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
                *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryLoadLayout",
            (name, table),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryLoadLayoutInternal(
        &mut self,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryLoadLayoutInternal",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValueTypeIsAssignableFrom(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
        valueType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ValueTypeIsAssignableFrom",
            (layoutName, valueType),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputControlLayout_ControlItem {
    pub _name_k__BackingField: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub _layout_k__BackingField: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub _variants_k__BackingField: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub _useStateFrom_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _displayName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _shortDisplayName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _usages_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub _aliases_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub _parameters_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::NamedValue,
    >,
    pub _processors_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
    >,
    pub _offset_k__BackingField: u32,
    pub _bit_k__BackingField: u32,
    pub _sizeInBits_k__BackingField: u32,
    pub _format_k__BackingField: crate::UnityEngine::InputSystem::Utilities::FourCC,
    pub _flags_k__BackingField: crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags,
    pub _arraySize_k__BackingField: i32,
    pub _defaultState_k__BackingField: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    pub _minValue_k__BackingField: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    pub _maxValue_k__BackingField: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/ControlItem"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem {
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItem+Flags"
    )]
    pub type Flags = crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags;
    pub fn Merge(
        &mut self,
        other: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Merge",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_aliases(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_aliases", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_arraySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_arraySize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bit(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bit",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_defaultState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_displayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_displayName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dontReset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_dontReset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flags",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_format",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isArray(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isArray",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isFirstDefinedInThisLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isFirstDefinedInThisLayout",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isModifyingExistingControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isModifyingExistingControl",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isNoisy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isNoisy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSynthetic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isSynthetic",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_layout",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_minValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_name",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_offset(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_offset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_parameters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_processors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_processors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shortDisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_shortDisplayName",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sizeInBits(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sizeInBits",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_usages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_usages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useStateFrom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_useStateFrom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_variants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_variants",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_aliases(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_aliases",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_arraySize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_arraySize",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bit(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bit",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultState(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_defaultState",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_displayName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_displayName",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_dontReset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_dontReset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flags(
        &mut self,
        value: crate::UnityEngine::InputSystem::Layouts::ControlItem_InputControlLayout_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_flags",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_format(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_format",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isFirstDefinedInThisLayout(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isFirstDefinedInThisLayout",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isModifyingExistingControl(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isModifyingExistingControl",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isNoisy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isSynthetic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isSynthetic",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_layout(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_layout",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxValue(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_maxValue",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minValue(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_minValue",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_name",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_offset(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_offset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_parameters(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NamedValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_parameters",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_processors(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::NameAndParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_processors",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_shortDisplayName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_shortDisplayName",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sizeInBits(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_sizeInBits",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_usages(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_usages",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useStateFrom(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_useStateFrom",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_variants(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_variants",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlLayout_ControlItemJson {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub layout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub variants: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub usage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub alias: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub useStateFrom: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub offset: u32,
    pub bit: u32,
    pub sizeInBits: u32,
    pub format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub arraySize: i32,
    pub usages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub aliases: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub processors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub shortDisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub noisy: bool,
    pub dontReset: bool,
    pub synthetic: bool,
    pub defaultState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub minValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub maxValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/ControlItemJson"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson {
    pub fn FromControlItems(
        items: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromControlItems", (items))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem = __cordl_object
            .invoke("ToLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+ControlItemJson")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputControlLayout_Flags {
    CanRunInBackground = 8i32,
    CanRunInBackgroundIsSet = 16i32,
    HideInUI = 2i32,
    IsGenericTypeOfDevice = 1i32,
    IsNoisy = 32i32,
    IsOverride = 4i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_Flags =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputControlLayout_LayoutJson {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub extend: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub extendMultiple: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub beforeRender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub runInBackground: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub commonUsages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub variant: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub isGenericTypeOfDevice: bool,
    pub hideInUI: bool,
    pub controls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItemJson,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/LayoutJson"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJson")]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson {
    pub fn FromLayout(
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJson = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromLayout", (layout))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToLayout", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputControlLayout_LayoutJsonNameAndDescriptorOnly {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub extend: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub extendMultiple: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub device: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher_MatcherJson,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly
    => "UnityEngine.InputSystem.Layouts"
    ."InputControlLayout/LayoutJsonNameAndDescriptorOnly"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutJsonNameAndDescriptorOnly"
)]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutJsonNameAndDescriptorOnly {}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlLayout_LayoutNotFoundException {
    __cordl_parent: crate::System::Exception,
    pub _layout_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException =>
    "UnityEngine.InputSystem.Layouts"."InputControlLayout/LayoutNotFoundException"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
impl crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Exception3(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerException))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, message))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext4(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString2(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Exception3(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerException))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext4(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_layout", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputControlLayout+LayoutNotFoundException"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Layouts::InputControlLayout_LayoutNotFoundException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
