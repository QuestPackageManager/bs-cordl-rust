#[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct AddInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::AddInstruction =>
    "System.Linq.Expressions.Interpreter"."AddInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::AddInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::AddInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction")]
impl crate::System::Linq::Expressions::Interpreter::AddInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction+AddDouble")]
    pub type AddDouble = crate::GlobalNamespace::AddInstruction_AddDouble;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction+AddInt16")]
    pub type AddInt16 = crate::GlobalNamespace::AddInstruction_AddInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction+AddInt32")]
    pub type AddInt32 = crate::GlobalNamespace::AddInstruction_AddInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction+AddInt64")]
    pub type AddInt64 = crate::GlobalNamespace::AddInstruction_AddInt64;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction+AddSingle")]
    pub type AddSingle = crate::GlobalNamespace::AddInstruction_AddSingle;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction+AddUInt16")]
    pub type AddUInt16 = crate::GlobalNamespace::AddInstruction_AddUInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction+AddUInt32")]
    pub type AddUInt32 = crate::GlobalNamespace::AddInstruction_AddUInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction+AddUInt64")]
    pub type AddUInt64 = crate::GlobalNamespace::AddInstruction_AddUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::AddInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
