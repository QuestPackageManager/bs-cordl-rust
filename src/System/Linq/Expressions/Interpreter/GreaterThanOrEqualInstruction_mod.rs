#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct GreaterThanOrEqualInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _nullValue: *mut crate::System::Object,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction =>
    "System.Linq.Expressions.Interpreter"."GreaterThanOrEqualInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction")]
impl crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualByte"
    )]
    pub type GreaterThanOrEqualByte = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSingle"
    )]
    pub type GreaterThanOrEqualSingle = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualInt16"
    )]
    pub type GreaterThanOrEqualInt16 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSByte"
    )]
    pub type GreaterThanOrEqualSByte = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualInt64"
    )]
    pub type GreaterThanOrEqualInt64 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualUInt16"
    )]
    pub type GreaterThanOrEqualUInt16 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualChar"
    )]
    pub type GreaterThanOrEqualChar = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualChar;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualUInt32"
    )]
    pub type GreaterThanOrEqualUInt32 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualUInt64"
    )]
    pub type GreaterThanOrEqualUInt64 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualUInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualInt32"
    )]
    pub type GreaterThanOrEqualInt32 = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualDouble"
    )]
    pub type GreaterThanOrEqualDouble = crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualDouble;
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
    pub fn get_ConsumedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ConsumedStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        nullValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nullValue))?;
        Ok(__cordl_ret)
    }
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nullValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nullValue))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
