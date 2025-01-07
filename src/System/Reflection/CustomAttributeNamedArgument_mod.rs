#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CustomAttributeNamedArgument {
    pub _TypedValue_k__BackingField: crate::System::Reflection::CustomAttributeTypedArgument,
    pub _IsField_k__BackingField: bool,
    pub _MemberName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _lazyMemberInfo: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::MemberInfo,
    >,
}
#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Reflection::CustomAttributeNamedArgument {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "CustomAttributeNamedArgument";
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
#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Reflection::CustomAttributeNamedArgument {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Reflection::CustomAttributeNamedArgument {
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
#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Reflection::CustomAttributeNamedArgument {
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
#[cfg(feature = "System+Reflection+CustomAttributeNamedArgument")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Reflection::CustomAttributeNamedArgument {
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
    pub fn op_Equality(
        left: crate::System::Reflection::CustomAttributeNamedArgument,
        right: crate::System::Reflection::CustomAttributeNamedArgument,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::System::Reflection::CustomAttributeNamedArgument,
        right: crate::System::Reflection::CustomAttributeNamedArgument,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
