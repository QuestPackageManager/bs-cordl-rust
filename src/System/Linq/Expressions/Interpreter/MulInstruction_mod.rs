#[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct MulInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::MulInstruction =>
    "System.Linq.Expressions.Interpreter"."MulInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::MulInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::MulInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction")]
impl crate::System::Linq::Expressions::Interpreter::MulInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction+MulDouble")]
    pub type MulDouble = crate::GlobalNamespace::MulInstruction_MulDouble;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction+MulInt16")]
    pub type MulInt16 = crate::GlobalNamespace::MulInstruction_MulInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction+MulInt32")]
    pub type MulInt32 = crate::GlobalNamespace::MulInstruction_MulInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction+MulInt64")]
    pub type MulInt64 = crate::GlobalNamespace::MulInstruction_MulInt64;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction+MulSingle")]
    pub type MulSingle = crate::GlobalNamespace::MulInstruction_MulSingle;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction+MulUInt16")]
    pub type MulUInt16 = crate::GlobalNamespace::MulInstruction_MulUInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction+MulUInt32")]
    pub type MulUInt32 = crate::GlobalNamespace::MulInstruction_MulUInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction+MulUInt64")]
    pub type MulUInt64 = crate::GlobalNamespace::MulInstruction_MulUInt64;
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
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::MulInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
