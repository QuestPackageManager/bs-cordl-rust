#[cfg(feature = "UnityEngine+InputSystem+InputControlPath")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlPath {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputControlPath =>
    "UnityEngine.InputSystem"."InputControlPath"
);
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
    #[cfg(feature = "UnityEngine+InputSystem+InputControlPath+_Parse_d__34")]
    pub type _Parse_d__34 = crate::UnityEngine::InputSystem::InputControlPath__Parse_d__34;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlPath+__c")]
    pub type __c = crate::UnityEngine::InputSystem::InputControlPath___c;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputControlPath_HumanReadableStringOptions {
    None = 0i32,
    OmitDevice = 2i32,
    UseShortNames = 4i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+HumanReadableStringOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlPath_HumanReadableStringOptions =>
    "UnityEngine.InputSystem"."InputControlPath/HumanReadableStringOptions"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+ParsedPathComponent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlPath_ParsedPathComponent {
    pub m_Layout: crate::UnityEngine::InputSystem::Utilities::Substring,
    pub m_Usages: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Utilities::Substring,
    >,
    pub m_Name: crate::UnityEngine::InputSystem::Utilities::Substring,
    pub m_DisplayName: crate::UnityEngine::InputSystem::Utilities::Substring,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+ParsedPathComponent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent =>
    "UnityEngine.InputSystem"."InputControlPath/ParsedPathComponent"
);
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
    #[cfg(feature = "UnityEngine+InputSystem+InputControlPath+ParsedPathComponent+__c")]
    pub type __c = crate::UnityEngine::InputSystem::ParsedPathComponent_InputControlPath___c;
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
    pub fn ToHumanReadableString(
        &mut self,
        parentLayoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentControlPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        referencedLayoutName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        controlPath: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_usages", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathComponentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputControlPath_PathComponentType {
    DisplayName = 1i32,
    Layout = 3i32,
    Name = 0i32,
    Usage = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathComponentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlPath_PathComponentType =>
    "UnityEngine.InputSystem"."InputControlPath/PathComponentType"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathParser")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlPath_PathParser {
    pub path: *mut quest_hook::libil2cpp::Il2CppString,
    pub length: i32,
    pub leftIndexInPath: i32,
    pub rightIndexInPath: i32,
    pub current: crate::UnityEngine::InputSystem::InputControlPath_ParsedPathComponent,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlPath+PathParser")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlPath_PathParser => "UnityEngine.InputSystem"
    ."InputControlPath/PathParser"
);
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
