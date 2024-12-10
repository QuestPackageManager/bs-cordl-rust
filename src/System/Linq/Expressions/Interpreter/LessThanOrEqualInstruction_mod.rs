#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct LessThanOrEqualInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _nullValue: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction =>
    "System.Linq.Expressions.Interpreter"."LessThanOrEqualInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction")]
impl crate::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualByte"
    )]
    pub type LessThanOrEqualByte = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualChar"
    )]
    pub type LessThanOrEqualChar = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualDouble"
    )]
    pub type LessThanOrEqualDouble = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualDouble;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualInt16"
    )]
    pub type LessThanOrEqualInt16 = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualInt32"
    )]
    pub type LessThanOrEqualInt32 = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualInt64"
    )]
    pub type LessThanOrEqualInt64 = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualSByte"
    )]
    pub type LessThanOrEqualSByte = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualSingle"
    )]
    pub type LessThanOrEqualSingle = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualUInt16"
    )]
    pub type LessThanOrEqualUInt16 = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualUInt32"
    )]
    pub type LessThanOrEqualUInt32 = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualUInt64"
    )]
    pub type LessThanOrEqualUInt64 = crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualUInt64;
    pub fn New(
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nullValue))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nullValue))?;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
