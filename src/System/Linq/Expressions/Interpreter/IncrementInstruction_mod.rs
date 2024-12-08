#[cfg(feature = "System+Linq+Expressions+Interpreter+IncrementInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct IncrementInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IncrementInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::IncrementInstruction =>
    "System.Linq.Expressions.Interpreter"."IncrementInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+IncrementInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::IncrementInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IncrementInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::IncrementInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IncrementInstruction")]
impl crate::System::Linq::Expressions::Interpreter::IncrementInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+IncrementInstruction+IncrementInt16"
    )]
    pub type IncrementInt16 = crate::GlobalNamespace::IncrementInstruction_IncrementInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+IncrementInstruction+IncrementSingle"
    )]
    pub type IncrementSingle = crate::GlobalNamespace::IncrementInstruction_IncrementSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+IncrementInstruction+IncrementDouble"
    )]
    pub type IncrementDouble = crate::GlobalNamespace::IncrementInstruction_IncrementDouble;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+IncrementInstruction+IncrementUInt16"
    )]
    pub type IncrementUInt16 = crate::GlobalNamespace::IncrementInstruction_IncrementUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+IncrementInstruction+IncrementUInt32"
    )]
    pub type IncrementUInt32 = crate::GlobalNamespace::IncrementInstruction_IncrementUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+IncrementInstruction+IncrementUInt64"
    )]
    pub type IncrementUInt64 = crate::GlobalNamespace::IncrementInstruction_IncrementUInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+IncrementInstruction+IncrementInt32"
    )]
    pub type IncrementInt32 = crate::GlobalNamespace::IncrementInstruction_IncrementInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+IncrementInstruction+IncrementInt64"
    )]
    pub type IncrementInt64 = crate::GlobalNamespace::IncrementInstruction_IncrementInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+IncrementInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::IncrementInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}