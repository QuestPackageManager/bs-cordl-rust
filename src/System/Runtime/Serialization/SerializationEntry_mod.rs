#[cfg(feature = "System+Runtime+Serialization+SerializationEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SerializationEntry {
    pub _name: *mut crate::System::String,
    pub _value: *mut crate::System::Object,
    pub _type: *mut crate::System::Type,
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
        entryName: *mut crate::System::String,
        entryValue: *mut crate::System::Object,
        entryType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (entryName, entryValue, entryType),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Name",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
