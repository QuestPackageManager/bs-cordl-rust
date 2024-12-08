#[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct MulOvfInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::MulOvfInstruction =>
    "System.Linq.Expressions.Interpreter"."MulOvfInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::MulOvfInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::MulOvfInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction")]
impl crate::System::Linq::Expressions::Interpreter::MulOvfInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction+MulOvfInt16")]
    pub type MulOvfInt16 = crate::GlobalNamespace::MulOvfInstruction_MulOvfInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction+MulOvfInt32")]
    pub type MulOvfInt32 = crate::GlobalNamespace::MulOvfInstruction_MulOvfInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction+MulOvfInt64")]
    pub type MulOvfInt64 = crate::GlobalNamespace::MulOvfInstruction_MulOvfInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction+MulOvfUInt16"
    )]
    pub type MulOvfUInt16 = crate::GlobalNamespace::MulOvfInstruction_MulOvfUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction+MulOvfUInt32"
    )]
    pub type MulOvfUInt32 = crate::GlobalNamespace::MulOvfInstruction_MulOvfUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction+MulOvfUInt64"
    )]
    pub type MulOvfUInt64 = crate::GlobalNamespace::MulOvfInstruction_MulOvfUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+MulOvfInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::MulOvfInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
