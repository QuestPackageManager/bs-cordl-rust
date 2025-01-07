#[cfg(feature = "UnityEngine+InputSystem+InputControlPath")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlPath {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlPath {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlPath";
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputControlPath {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputControlPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath")]
impl crate::UnityEngine::InputSystem::InputControlPath {
    pub const DoubleWildcard: &'static str = "**";
    pub const Separator: char = '/';
    pub const SeparatorReplacement: char = ' ';
    pub const Wildcard: &'static str = "*";
    #[cfg(
        feature = "UnityEngine+InputSystem+InputControlPath+HumanReadableStringOptions"
    )]
    pub type HumanReadableStringOptions = crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlPath+ParsedPathComponent")]
    pub type ParsedPathComponent = crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathComponentType")]
    pub type PathComponentType = crate::UnityEngine::InputSystem::InputControlPath_PathComponentType;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathParser")]
    pub type PathParser = crate::UnityEngine::InputSystem::InputControlPath_PathParser;
    pub fn CleanSlashes(
        pathComponent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CleanSlashes", (pathComponent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine(
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (parent, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn ControlLayoutMatchesPathComponent(
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
        parser: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlPath_PathParser,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ControlLayoutMatchesPathComponent", (controlItem, parser))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControlLayoutRecursive_Il2CppString0(
        parser: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlPath_PathParser,
        >,
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindControlLayoutRecursive", (parser, layoutName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControlLayoutRecursive_InputControlLayout1(
        parser: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlPath_PathParser,
        >,
        layout: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindControlLayoutRecursive", (parser, layout))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchByUsageAtDeviceRootRecursive<TControl>(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
        matches: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
        >,
        matchMultiple: bool,
    ) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TControl = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MatchByUsageAtDeviceRootRecursive",
                (device, path, indexInPath, matches, matchMultiple),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchChildrenRecursive<TControl>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
        matches: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
        >,
        matchMultiple: bool,
    ) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TControl = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MatchChildrenRecursive",
                (control, path, indexInPath, matches, matchMultiple),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchControlComponent(
        expectedControlComponent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent,
        >,
        controlItem: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout_ControlItem,
        >,
        matchAlias: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MatchControlComponent",
                (expectedControlComponent, controlItem, matchAlias),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchControlsRecursive<TControl>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
        matches: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
        >,
        matchMultiple: bool,
    ) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TControl = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MatchControlsRecursive",
                (control, path, indexInPath, matches, matchMultiple),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchPathComponent(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: quest_hook::libil2cpp::ByRefMut<i32>,
        componentType: crate::UnityEngine::InputSystem::InputControlPath_PathComponentType,
        startIndexInComponent: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MatchPathComponent",
                (component, path, indexInPath, componentType, startIndexInComponent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Matches(
        expected: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Matches", (expected, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesPrefix(
        expected: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesPrefix", (expected, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesRecursive(
        parser: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlPath_PathParser,
        >,
        currentControl: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        prefixOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesRecursive", (parser, currentControl, prefixOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Parse", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn PathComponentCanYieldMultipleMatches(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PathComponentCanYieldMultipleMatches", (path, indexInPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringMatches(
        str: crate::UnityEngine::InputSystem::Utilities::Substring,
        matchTo: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringMatches", (str, matchTo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHumanReadableString_ByRefMut_ByRefMut_InputControlPath_HumanReadableStringOptions_InputControl1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deviceLayoutName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        controlPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        options: crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ToHumanReadableString",
                (path, deviceLayoutName, controlPath, options, control),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHumanReadableString_InputControlPath_HumanReadableStringOptions_InputControl0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHumanReadableString", (path, options, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindChild_InputControl_Il2CppString_i32_0(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindChild", (control, path, indexInPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindChild_InputControl_Il2CppString_i32_1<TControl>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
    ) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TControl = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindChild", (control, path, indexInPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindControl_InputControl_Il2CppString_i32_0(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindControl", (control, path, indexInPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindControl_InputControl_Il2CppString_i32_1<TControl>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
    ) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TControl = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindControl", (control, path, indexInPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindControls_ByRefMut_i32_1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        matches: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        >,
        indexInPath: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindControls", (control, path, matches, indexInPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindControls_i32_0(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindControls", (control, path, indexInPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindControls_i32_ByRefMut2<TControl>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indexInPath: i32,
        matches: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindControls", (control, path, indexInPath, matches))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetControlLayout(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetControlLayout", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDeviceLayout(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetDeviceLayout", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDeviceUsages(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetDeviceUsages", (path))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputControlPath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+HumanReadableStringOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputControlPath_HumanReadableStringOptions {
    #[default]
    None = 0i32,
    OmitDevice = 2i32,
    UseShortNames = 4i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+HumanReadableStringOptions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "HumanReadableStringOptions";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+ParsedPathComponent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputControlPath_ParsedPathComponent {
    pub m_Layout: crate::UnityEngine::InputSystem::Utilities::Substring,
    pub m_Usages: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Utilities::Substring,
    >,
    pub m_Name: crate::UnityEngine::InputSystem::Utilities::Substring,
    pub m_DisplayName: crate::UnityEngine::InputSystem::Utilities::Substring,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+ParsedPathComponent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "ParsedPathComponent";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+ParsedPathComponent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+ParsedPathComponent")]
impl crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent {
    pub fn ComparePathElementToString(
        pathElement: crate::UnityEngine::InputSystem::Utilities::Substring,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComparePathElementToString", (pathElement, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn Matches(
        &mut self,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Matches",
            (control),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHumanReadableString_Il2CppString_Il2CppString_ByRefMut_ByRefMut_InputControlPath_HumanReadableStringOptions0(
        &mut self,
        parentLayoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentControlPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        referencedLayoutName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        controlPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        options: crate::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToHumanReadableString",
            (
                parentLayoutName,
                parentControlPath,
                referencedLayoutName,
                controlPath,
                options,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHumanReadableString_Substring1(
        substring: crate::UnityEngine::InputSystem::Utilities::Substring,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHumanReadableString", (substring))?;
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
    pub fn get_isDoubleWildcard(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isDoubleWildcard",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isWildcard(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isWildcard",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_layout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_usages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_usages", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathComponentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputControlPath_PathComponentType {
    #[default]
    DisplayName = 1i32,
    Layout = 3i32,
    Name = 0i32,
    Usage = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathComponentType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlPath_PathComponentType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "PathComponentType";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlPath_PathComponentType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlPath_PathComponentType {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlPath_PathComponentType {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlPath_PathComponentType {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathParser")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputControlPath_PathParser {
    pub path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub length: i32,
    pub leftIndexInPath: i32,
    pub rightIndexInPath: i32,
    pub current: crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlPath_PathParser {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "PathParser";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlPath_PathParser {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlPath_PathParser {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlPath_PathParser {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlPath_PathParser {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathParser")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlPath_PathParser {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathParser")]
impl crate::UnityEngine::InputSystem::InputControlPath_PathParser {
    pub fn MoveToNextComponent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveToNextComponent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseComponentPart(
        &mut self,
        terminator: char,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::Substring,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::Substring = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseComponentPart",
            (terminator),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (path),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isAtEnd(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isAtEnd",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
