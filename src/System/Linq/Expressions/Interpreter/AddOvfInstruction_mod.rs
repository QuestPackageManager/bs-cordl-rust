#[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct AddOvfInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::AddOvfInstruction =>
    "System.Linq.Expressions.Interpreter"."AddOvfInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::AddOvfInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::AddOvfInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction")]
impl crate::System::Linq::Expressions::Interpreter::AddOvfInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction+AddOvfInt16")]
    pub type AddOvfInt16 = crate::GlobalNamespace::AddOvfInstruction_AddOvfInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction+AddOvfInt32")]
    pub type AddOvfInt32 = crate::GlobalNamespace::AddOvfInstruction_AddOvfInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction+AddOvfInt64")]
    pub type AddOvfInt64 = crate::GlobalNamespace::AddOvfInstruction_AddOvfInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction+AddOvfUInt16"
    )]
    pub type AddOvfUInt16 = crate::GlobalNamespace::AddOvfInstruction_AddOvfUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction+AddOvfUInt32"
    )]
    pub type AddOvfUInt32 = crate::GlobalNamespace::AddOvfInstruction_AddOvfUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction+AddOvfUInt64"
    )]
    pub type AddOvfUInt64 = crate::GlobalNamespace::AddOvfInstruction_AddOvfUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+AddOvfInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::AddOvfInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
