#[cfg(feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct NotEqualInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::NotEqualInstruction =>
    "System.Linq.Expressions.Interpreter"."NotEqualInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::NotEqualInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::NotEqualInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction")]
impl crate::System::Linq::Expressions::Interpreter::NotEqualInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualBoolean"
    )]
    pub type NotEqualBoolean = crate::GlobalNamespace::NotEqualInstruction_NotEqualBoolean;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualByte"
    )]
    pub type NotEqualByte = crate::GlobalNamespace::NotEqualInstruction_NotEqualByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualByteLiftedToNull"
    )]
    pub type NotEqualByteLiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualByteLiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualChar"
    )]
    pub type NotEqualChar = crate::GlobalNamespace::NotEqualInstruction_NotEqualChar;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualCharLiftedToNull"
    )]
    pub type NotEqualCharLiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualCharLiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualDouble"
    )]
    pub type NotEqualDouble = crate::GlobalNamespace::NotEqualInstruction_NotEqualDouble;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualDoubleLiftedToNull"
    )]
    pub type NotEqualDoubleLiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualDoubleLiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualInt16"
    )]
    pub type NotEqualInt16 = crate::GlobalNamespace::NotEqualInstruction_NotEqualInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualInt16LiftedToNull"
    )]
    pub type NotEqualInt16LiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualInt16LiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualInt32"
    )]
    pub type NotEqualInt32 = crate::GlobalNamespace::NotEqualInstruction_NotEqualInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualInt32LiftedToNull"
    )]
    pub type NotEqualInt32LiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualInt32LiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualInt64"
    )]
    pub type NotEqualInt64 = crate::GlobalNamespace::NotEqualInstruction_NotEqualInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualInt64LiftedToNull"
    )]
    pub type NotEqualInt64LiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualInt64LiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualReference"
    )]
    pub type NotEqualReference = crate::GlobalNamespace::NotEqualInstruction_NotEqualReference;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSByte"
    )]
    pub type NotEqualSByte = crate::GlobalNamespace::NotEqualInstruction_NotEqualSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSByteLiftedToNull"
    )]
    pub type NotEqualSByteLiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualSByteLiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSingle"
    )]
    pub type NotEqualSingle = crate::GlobalNamespace::NotEqualInstruction_NotEqualSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualSingleLiftedToNull"
    )]
    pub type NotEqualSingleLiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualSingleLiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualUInt16"
    )]
    pub type NotEqualUInt16 = crate::GlobalNamespace::NotEqualInstruction_NotEqualUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualUInt16LiftedToNull"
    )]
    pub type NotEqualUInt16LiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualUInt16LiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualUInt32"
    )]
    pub type NotEqualUInt32 = crate::GlobalNamespace::NotEqualInstruction_NotEqualUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualUInt32LiftedToNull"
    )]
    pub type NotEqualUInt32LiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualUInt32LiftedToNull;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualUInt64"
    )]
    pub type NotEqualUInt64 = crate::GlobalNamespace::NotEqualInstruction_NotEqualUInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction+NotEqualUInt64LiftedToNull"
    )]
    pub type NotEqualUInt64LiftedToNull = crate::GlobalNamespace::NotEqualInstruction_NotEqualUInt64LiftedToNull;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotEqualInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::NotEqualInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
