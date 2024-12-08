#[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct NotInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::NotInstruction =>
    "System.Linq.Expressions.Interpreter"."NotInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::NotInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::NotInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction")]
impl crate::System::Linq::Expressions::Interpreter::NotInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotByte")]
    pub type NotByte = crate::GlobalNamespace::NotInstruction_NotByte;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotUInt16")]
    pub type NotUInt16 = crate::GlobalNamespace::NotInstruction_NotUInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotInt16")]
    pub type NotInt16 = crate::GlobalNamespace::NotInstruction_NotInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotUInt64")]
    pub type NotUInt64 = crate::GlobalNamespace::NotInstruction_NotUInt64;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotSByte")]
    pub type NotSByte = crate::GlobalNamespace::NotInstruction_NotSByte;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotInt32")]
    pub type NotInt32 = crate::GlobalNamespace::NotInstruction_NotInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotUInt32")]
    pub type NotUInt32 = crate::GlobalNamespace::NotInstruction_NotUInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotBoolean")]
    pub type NotBoolean = crate::GlobalNamespace::NotInstruction_NotBoolean;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction+NotInt64")]
    pub type NotInt64 = crate::GlobalNamespace::NotInstruction_NotInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+NotInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::NotInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
