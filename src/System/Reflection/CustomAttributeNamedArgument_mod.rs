#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CustomAttributeNamedArgument {
    pub _TypedValue_k__BackingField: crate::System::Reflection::CustomAttributeTypedArgument,
    pub _IsField_k__BackingField: bool,
    pub _MemberName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _attributeType: *mut crate::System::Type,
    pub _lazyMemberInfo: *mut crate::System::Reflection::MemberInfo,
}
#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::CustomAttributeNamedArgument
    => "System.Reflection"."CustomAttributeNamedArgument"
);
#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Reflection::CustomAttributeNamedArgument {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
impl crate::System::Reflection::CustomAttributeNamedArgument {
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_MemberInfo_CustomAttributeTypedArgument2(
        &mut self,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        typedArgument: crate::System::Reflection::CustomAttributeTypedArgument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (memberInfo, typedArgument),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_MemberInfo_Il2CppObject1(
        &mut self,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (memberInfo, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_Il2CppString__cordl_bool_CustomAttributeTypedArgument0(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isField: bool,
        typedValue: crate::System::Reflection::CustomAttributeTypedArgument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (attributeType, memberName, isField, typedValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsField(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsField",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MemberInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MemberInfo,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_MemberInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MemberName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_MemberName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Reflection::CustomAttributeTypedArgument,
    > {
        let __cordl_ret: crate::System::Reflection::CustomAttributeTypedArgument = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TypedValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
