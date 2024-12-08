#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct NegateCheckedInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::NegateCheckedInstruction =>
    "System.Linq.Expressions.Interpreter"."NegateCheckedInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::NegateCheckedInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::NegateCheckedInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction")]
impl crate::System::Linq::Expressions::Interpreter::NegateCheckedInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt16"
    )]
    pub type NegateCheckedInt16 = crate::GlobalNamespace::NegateCheckedInstruction_NegateCheckedInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt32"
    )]
    pub type NegateCheckedInt32 = crate::GlobalNamespace::NegateCheckedInstruction_NegateCheckedInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt64"
    )]
    pub type NegateCheckedInt64 = crate::GlobalNamespace::NegateCheckedInstruction_NegateCheckedInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::NegateCheckedInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
