#[cfg(feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct ExclusiveOrInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "ExclusiveOrInstruction";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction")]
impl crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrBoolean"
    )]
    pub type ExclusiveOrBoolean = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrBoolean;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrByte"
    )]
    pub type ExclusiveOrByte = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrInt16"
    )]
    pub type ExclusiveOrInt16 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrInt32"
    )]
    pub type ExclusiveOrInt32 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrInt64"
    )]
    pub type ExclusiveOrInt64 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrSByte"
    )]
    pub type ExclusiveOrSByte = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrUInt16"
    )]
    pub type ExclusiveOrUInt16 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrUInt32"
    )]
    pub type ExclusiveOrUInt32 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction+ExclusiveOrUInt64"
    )]
    pub type ExclusiveOrUInt64 = crate::GlobalNamespace::ExclusiveOrInstruction_ExclusiveOrUInt64;
    pub fn Create(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (_cordl_type))?;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+ExclusiveOrInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ExclusiveOrInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
