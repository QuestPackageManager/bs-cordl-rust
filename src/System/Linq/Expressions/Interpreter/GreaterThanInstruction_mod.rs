#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct GreaterThanInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _nullValue: *mut crate::System::Object,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::GreaterThanInstruction =>
    "System.Linq.Expressions.Interpreter"."GreaterThanInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction")]
impl crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanByte"
    )]
    pub type GreaterThanByte = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanUInt32"
    )]
    pub type GreaterThanUInt32 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanInt16"
    )]
    pub type GreaterThanInt16 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanChar"
    )]
    pub type GreaterThanChar = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanChar;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanInt32"
    )]
    pub type GreaterThanInt32 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanInt64"
    )]
    pub type GreaterThanInt64 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSByte"
    )]
    pub type GreaterThanSByte = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanUInt64"
    )]
    pub type GreaterThanUInt64 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanUInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanSingle"
    )]
    pub type GreaterThanSingle = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanUInt16"
    )]
    pub type GreaterThanUInt16 = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction+GreaterThanDouble"
    )]
    pub type GreaterThanDouble = crate::GlobalNamespace::GreaterThanInstruction_GreaterThanDouble;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+GreaterThanInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::GreaterThanInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
