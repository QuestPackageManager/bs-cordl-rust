#[cfg(feature = "System+Linq+Expressions+Interpreter+DecrementInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct DecrementInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DecrementInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::DecrementInstruction =>
    "System.Linq.Expressions.Interpreter"."DecrementInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+DecrementInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::DecrementInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DecrementInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::DecrementInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DecrementInstruction")]
impl crate::System::Linq::Expressions::Interpreter::DecrementInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+DecrementInstruction+DecrementDouble"
    )]
    pub type DecrementDouble = crate::GlobalNamespace::DecrementInstruction_DecrementDouble;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+DecrementInstruction+DecrementInt16"
    )]
    pub type DecrementInt16 = crate::GlobalNamespace::DecrementInstruction_DecrementInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+DecrementInstruction+DecrementInt32"
    )]
    pub type DecrementInt32 = crate::GlobalNamespace::DecrementInstruction_DecrementInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+DecrementInstruction+DecrementInt64"
    )]
    pub type DecrementInt64 = crate::GlobalNamespace::DecrementInstruction_DecrementInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+DecrementInstruction+DecrementSingle"
    )]
    pub type DecrementSingle = crate::GlobalNamespace::DecrementInstruction_DecrementSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+DecrementInstruction+DecrementUInt16"
    )]
    pub type DecrementUInt16 = crate::GlobalNamespace::DecrementInstruction_DecrementUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+DecrementInstruction+DecrementUInt32"
    )]
    pub type DecrementUInt32 = crate::GlobalNamespace::DecrementInstruction_DecrementUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+DecrementInstruction+DecrementUInt64"
    )]
    pub type DecrementUInt64 = crate::GlobalNamespace::DecrementInstruction_DecrementUInt64;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConsumedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ConsumedStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_InstructionName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedStack", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DecrementInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::DecrementInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
