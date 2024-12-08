#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct NegateInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::NegateInstruction =>
    "System.Linq.Expressions.Interpreter"."NegateInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::NegateInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::NegateInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction")]
impl crate::System::Linq::Expressions::Interpreter::NegateInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction+NegateInt32")]
    pub type NegateInt32 = crate::GlobalNamespace::NegateInstruction_NegateInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NegateInstruction+NegateSingle"
    )]
    pub type NegateSingle = crate::GlobalNamespace::NegateInstruction_NegateSingle;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction+NegateInt16")]
    pub type NegateInt16 = crate::GlobalNamespace::NegateInstruction_NegateInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NegateInstruction+NegateDouble"
    )]
    pub type NegateDouble = crate::GlobalNamespace::NegateInstruction_NegateDouble;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction+NegateInt64")]
    pub type NegateInt64 = crate::GlobalNamespace::NegateInstruction_NegateInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::NegateInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
