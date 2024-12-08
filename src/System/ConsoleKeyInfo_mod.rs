#[cfg(feature = "System+ConsoleKeyInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ConsoleKeyInfo {
    pub _keyChar: char,
    pub _key: crate::System::ConsoleKey,
    pub _mods: crate::System::ConsoleModifiers,
}
#[cfg(feature = "System+ConsoleKeyInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ConsoleKeyInfo => "System"
    ."ConsoleKeyInfo"
);
#[cfg(feature = "System+ConsoleKeyInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::ConsoleKeyInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ConsoleKeyInfo")]
impl crate::System::ConsoleKeyInfo {
    pub fn Equals_ConsoleKeyInfo1(
        &mut self,
        obj: crate::System::ConsoleKeyInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        keyChar: char,
        key: crate::System::ConsoleKey,
        shift: bool,
        alt: bool,
        control: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyChar, key, shift, alt, control),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKey> {
        let __cordl_ret: crate::System::ConsoleKey = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Key",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_KeyChar",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
