#[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct EqualInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::EqualInstruction =>
    "System.Linq.Expressions.Interpreter"."EqualInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::EqualInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::EqualInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction")]
impl crate::System::Linq::Expressions::Interpreter::EqualInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualBoolean")]
    pub type EqualBoolean = crate::GlobalNamespace::EqualInstruction_EqualBoolean;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualBooleanLiftedToNull"
    )]
    pub type EqualBooleanLiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualBooleanLiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualByte")]
    pub type EqualByte = crate::GlobalNamespace::EqualInstruction_EqualByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualByteLiftedToNull"
    )]
    pub type EqualByteLiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualByteLiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualChar")]
    pub type EqualChar = crate::GlobalNamespace::EqualInstruction_EqualChar;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualCharLiftedToNull"
    )]
    pub type EqualCharLiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualCharLiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualDouble")]
    pub type EqualDouble = crate::GlobalNamespace::EqualInstruction_EqualDouble;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualDoubleLiftedToNull"
    )]
    pub type EqualDoubleLiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualDoubleLiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualInt16")]
    pub type EqualInt16 = crate::GlobalNamespace::EqualInstruction_EqualInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualInt16LiftedToNull"
    )]
    pub type EqualInt16LiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualInt16LiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualInt32")]
    pub type EqualInt32 = crate::GlobalNamespace::EqualInstruction_EqualInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualInt32LiftedToNull"
    )]
    pub type EqualInt32LiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualInt32LiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualInt64")]
    pub type EqualInt64 = crate::GlobalNamespace::EqualInstruction_EqualInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualInt64LiftedToNull"
    )]
    pub type EqualInt64LiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualInt64LiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualReference"
    )]
    pub type EqualReference = crate::GlobalNamespace::EqualInstruction_EqualReference;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualSByte")]
    pub type EqualSByte = crate::GlobalNamespace::EqualInstruction_EqualSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualSByteLiftedToNull"
    )]
    pub type EqualSByteLiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualSByteLiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualSingle")]
    pub type EqualSingle = crate::GlobalNamespace::EqualInstruction_EqualSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualSingleLiftedToNull"
    )]
    pub type EqualSingleLiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualSingleLiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt16")]
    pub type EqualUInt16 = crate::GlobalNamespace::EqualInstruction_EqualUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt16LiftedToNull"
    )]
    pub type EqualUInt16LiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualUInt16LiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt32")]
    pub type EqualUInt32 = crate::GlobalNamespace::EqualInstruction_EqualUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt32LiftedToNull"
    )]
    pub type EqualUInt32LiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualUInt32LiftedToNull;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt64")]
    pub type EqualUInt64 = crate::GlobalNamespace::EqualInstruction_EqualUInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt64LiftedToNull"
    )]
    pub type EqualUInt64LiftedToNull = crate::GlobalNamespace::EqualInstruction_EqualUInt64LiftedToNull;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+EqualInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::EqualInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
