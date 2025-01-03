#[cfg(feature = "System+Runtime+Serialization+SerializationEntry")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SerializationEntry {
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SerializationEntry =>
    "System.Runtime.Serialization"."SerializationEntry"
);
#[cfg(feature = "System+Runtime+Serialization+SerializationEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::Serialization::SerializationEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationEntry")]
impl crate::System::Runtime::Serialization::SerializationEntry {
    pub fn _ctor(
        &mut self,
        entryName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        entryValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        entryType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (entryName, entryValue, entryType),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
