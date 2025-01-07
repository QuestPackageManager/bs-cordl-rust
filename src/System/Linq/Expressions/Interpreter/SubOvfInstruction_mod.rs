#[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct SubOvfInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::SubOvfInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "SubOvfInstruction";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::SubOvfInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::SubOvfInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction")]
impl crate::System::Linq::Expressions::Interpreter::SubOvfInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction+SubOvfInt16")]
    pub type SubOvfInt16 = crate::GlobalNamespace::SubOvfInstruction_SubOvfInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction+SubOvfInt32")]
    pub type SubOvfInt32 = crate::GlobalNamespace::SubOvfInstruction_SubOvfInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction+SubOvfInt64")]
    pub type SubOvfInt64 = crate::GlobalNamespace::SubOvfInstruction_SubOvfInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction+SubOvfUInt16"
    )]
    pub type SubOvfUInt16 = crate::GlobalNamespace::SubOvfInstruction_SubOvfUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction+SubOvfUInt32"
    )]
    pub type SubOvfUInt32 = crate::GlobalNamespace::SubOvfInstruction_SubOvfUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction+SubOvfUInt64"
    )]
    pub type SubOvfUInt64 = crate::GlobalNamespace::SubOvfInstruction_SubOvfUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+SubOvfInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::SubOvfInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
