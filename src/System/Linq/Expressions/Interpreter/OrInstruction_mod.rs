#[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct OrInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::OrInstruction =>
    "System.Linq.Expressions.Interpreter"."OrInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::OrInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::OrInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction")]
impl crate::System::Linq::Expressions::Interpreter::OrInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrBoolean")]
    pub type OrBoolean = crate::GlobalNamespace::OrInstruction_OrBoolean;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrByte")]
    pub type OrByte = crate::GlobalNamespace::OrInstruction_OrByte;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrInt16")]
    pub type OrInt16 = crate::GlobalNamespace::OrInstruction_OrInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrInt32")]
    pub type OrInt32 = crate::GlobalNamespace::OrInstruction_OrInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrInt64")]
    pub type OrInt64 = crate::GlobalNamespace::OrInstruction_OrInt64;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrSByte")]
    pub type OrSByte = crate::GlobalNamespace::OrInstruction_OrSByte;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrUInt16")]
    pub type OrUInt16 = crate::GlobalNamespace::OrInstruction_OrUInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrUInt32")]
    pub type OrUInt32 = crate::GlobalNamespace::OrInstruction_OrUInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction+OrUInt64")]
    pub type OrUInt64 = crate::GlobalNamespace::OrInstruction_OrUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+OrInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::OrInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
