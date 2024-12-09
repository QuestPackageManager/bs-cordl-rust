#[cfg(feature = "BGLib+DotnetExtension+CommandLine+ArgumentOption")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ArgumentOption {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub identifiers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub hint: *mut quest_hook::libil2cpp::Il2CppString,
    pub _cordl_type: crate::BGLib::DotnetExtension::CommandLine::ArgumentType,
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+ArgumentOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::DotnetExtension::CommandLine::ArgumentOption =>
    "BGLib.DotnetExtension.CommandLine"."ArgumentOption"
);
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+ArgumentOption")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BGLib::DotnetExtension::CommandLine::ArgumentOption {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BGLib+DotnetExtension+CommandLine+ArgumentOption")]
impl crate::BGLib::DotnetExtension::CommandLine::ArgumentOption {
    pub fn ValidateArgumentValue(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ValidateArgumentValue",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        hint: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: crate::BGLib::DotnetExtension::CommandLine::ArgumentType,
        identifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name, hint, _cordl_type, identifiers),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_expectsValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_expectsValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_required(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_required",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
