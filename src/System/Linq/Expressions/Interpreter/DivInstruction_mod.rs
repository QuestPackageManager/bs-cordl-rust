#[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct DivInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::DivInstruction =>
    "System.Linq.Expressions.Interpreter"."DivInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::DivInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::DivInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction")]
impl crate::System::Linq::Expressions::Interpreter::DivInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction+DivDouble")]
    pub type DivDouble = crate::GlobalNamespace::DivInstruction_DivDouble;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction+DivInt16")]
    pub type DivInt16 = crate::GlobalNamespace::DivInstruction_DivInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction+DivInt32")]
    pub type DivInt32 = crate::GlobalNamespace::DivInstruction_DivInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction+DivInt64")]
    pub type DivInt64 = crate::GlobalNamespace::DivInstruction_DivInt64;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction+DivSingle")]
    pub type DivSingle = crate::GlobalNamespace::DivInstruction_DivSingle;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction+DivUInt16")]
    pub type DivUInt16 = crate::GlobalNamespace::DivInstruction_DivUInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction+DivUInt32")]
    pub type DivUInt32 = crate::GlobalNamespace::DivInstruction_DivUInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction+DivUInt64")]
    pub type DivUInt64 = crate::GlobalNamespace::DivInstruction_DivUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+DivInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::DivInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
