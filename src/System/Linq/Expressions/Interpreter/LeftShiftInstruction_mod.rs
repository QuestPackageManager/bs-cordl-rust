#[cfg(feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct LeftShiftInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LeftShiftInstruction =>
    "System.Linq.Expressions.Interpreter"."LeftShiftInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::LeftShiftInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LeftShiftInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction")]
impl crate::System::Linq::Expressions::Interpreter::LeftShiftInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction+LeftShiftUInt16"
    )]
    pub type LeftShiftUInt16 = crate::GlobalNamespace::LeftShiftInstruction_LeftShiftUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction+LeftShiftSByte"
    )]
    pub type LeftShiftSByte = crate::GlobalNamespace::LeftShiftInstruction_LeftShiftSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction+LeftShiftInt64"
    )]
    pub type LeftShiftInt64 = crate::GlobalNamespace::LeftShiftInstruction_LeftShiftInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction+LeftShiftUInt32"
    )]
    pub type LeftShiftUInt32 = crate::GlobalNamespace::LeftShiftInstruction_LeftShiftUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction+LeftShiftInt16"
    )]
    pub type LeftShiftInt16 = crate::GlobalNamespace::LeftShiftInstruction_LeftShiftInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction+LeftShiftInt32"
    )]
    pub type LeftShiftInt32 = crate::GlobalNamespace::LeftShiftInstruction_LeftShiftInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction+LeftShiftUInt64"
    )]
    pub type LeftShiftUInt64 = crate::GlobalNamespace::LeftShiftInstruction_LeftShiftUInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction+LeftShiftByte"
    )]
    pub type LeftShiftByte = crate::GlobalNamespace::LeftShiftInstruction_LeftShiftByte;
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
    pub fn get_ProducedStack(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProducedStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LeftShiftInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LeftShiftInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
