#[cfg(feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct NullableMethodCallInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::NullableMethodCallInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "NullableMethodCallInstruction";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::NullableMethodCallInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::NullableMethodCallInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction")]
impl crate::System::Linq::Expressions::Interpreter::NullableMethodCallInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+EqualsClass"
    )]
    pub type EqualsClass = crate::GlobalNamespace::NullableMethodCallInstruction_EqualsClass;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+GetHashCodeClass"
    )]
    pub type GetHashCodeClass = crate::GlobalNamespace::NullableMethodCallInstruction_GetHashCodeClass;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+GetValue"
    )]
    pub type GetValue = crate::GlobalNamespace::NullableMethodCallInstruction_GetValue;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+GetValueOrDefault"
    )]
    pub type GetValueOrDefault = crate::GlobalNamespace::NullableMethodCallInstruction_GetValueOrDefault;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+GetValueOrDefault1"
    )]
    pub type GetValueOrDefault1 = crate::GlobalNamespace::NullableMethodCallInstruction_GetValueOrDefault1;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+HasValue"
    )]
    pub type HasValue = crate::GlobalNamespace::NullableMethodCallInstruction_HasValue;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+ToStringClass"
    )]
    pub type ToStringClass = crate::GlobalNamespace::NullableMethodCallInstruction_ToStringClass;
    pub fn Create(
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argCount: i32,
        mi: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (method, argCount, mi))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateGetValue() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreateGetValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConsumedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ConsumedStack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_InstructionName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedStack", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::NullableMethodCallInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
