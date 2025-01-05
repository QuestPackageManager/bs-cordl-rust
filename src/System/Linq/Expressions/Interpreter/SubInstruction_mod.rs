#[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct SubInstruction {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::Instruction,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::SubInstruction =>
    "System.Linq.Expressions.Interpreter"."SubInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::SubInstruction {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::Instruction,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::SubInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction")]
impl crate::System::Linq::Expressions::Interpreter::SubInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction+SubDouble")]
    pub type SubDouble = crate::GlobalNamespace::SubInstruction_SubDouble;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction+SubInt16")]
    pub type SubInt16 = crate::GlobalNamespace::SubInstruction_SubInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction+SubInt32")]
    pub type SubInt32 = crate::GlobalNamespace::SubInstruction_SubInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction+SubInt64")]
    pub type SubInt64 = crate::GlobalNamespace::SubInstruction_SubInt64;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction+SubSingle")]
    pub type SubSingle = crate::GlobalNamespace::SubInstruction_SubSingle;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction+SubUInt16")]
    pub type SubUInt16 = crate::GlobalNamespace::SubInstruction_SubUInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction+SubUInt32")]
    pub type SubUInt32 = crate::GlobalNamespace::SubInstruction_SubUInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction+SubUInt64")]
    pub type SubUInt64 = crate::GlobalNamespace::SubInstruction_SubUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::SubInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
