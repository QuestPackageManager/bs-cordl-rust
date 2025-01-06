#[cfg(feature = "System+Xml+Schema+Position")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position {
    pub symbol: i32,
    pub particle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Xml+Schema+Position")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Position =>
    "System.Xml.Schema"."Position"
);
#[cfg(feature = "System+Xml+Schema+Position")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::Position {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+Position")]
impl crate::System::Xml::Schema::Position {
    pub fn _ctor(
        &mut self,
        symbol: i32,
        particle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (symbol, particle),
        )?;
        Ok(__cordl_ret.into())
    }
}
