#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalDefinition")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LocalDefinition {
    pub _Index_k__BackingField: i32,
    pub _Parameter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::ParameterExpression,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalDefinition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LocalDefinition =>
    "System.Linq.Expressions.Interpreter"."LocalDefinition"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalDefinition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Linq::Expressions::Interpreter::LocalDefinition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LocalDefinition")]
impl crate::System::Linq::Expressions::Interpreter::LocalDefinition {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        localIndex: i32,
        parameter: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (localIndex, parameter),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Index",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Parameter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ParameterExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Parameter", ())?;
        Ok(__cordl_ret.into())
    }
}
