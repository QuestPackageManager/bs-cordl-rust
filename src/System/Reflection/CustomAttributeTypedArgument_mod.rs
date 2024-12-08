#[cfg(feature = "System+Reflection+CustomAttributeTypedArgument")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CustomAttributeTypedArgument {
    pub _ArgumentType_k__BackingField: *mut crate::System::Type,
    pub _Value_k__BackingField: *mut crate::System::Object,
}
#[cfg(feature = "System+Reflection+CustomAttributeTypedArgument")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::CustomAttributeTypedArgument
    => "System.Reflection"."CustomAttributeTypedArgument"
);
#[cfg(feature = "System+Reflection+CustomAttributeTypedArgument")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Reflection::CustomAttributeTypedArgument {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeTypedArgument")]
impl crate::System::Reflection::CustomAttributeTypedArgument {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
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
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString__cordl_bool1(
        &mut self,
        typed: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (typed),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object0(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_Object1(
        &mut self,
        argumentType: *mut crate::System::Type,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (argumentType, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ArgumentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_ret: *mut crate::System::Type = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ArgumentType",
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