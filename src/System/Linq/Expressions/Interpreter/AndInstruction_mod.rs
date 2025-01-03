#[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct AndInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::AndInstruction =>
    "System.Linq.Expressions.Interpreter"."AndInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::AndInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::AndInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction")]
impl crate::System::Linq::Expressions::Interpreter::AndInstruction {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndBoolean")]
    pub type AndBoolean = crate::GlobalNamespace::AndInstruction_AndBoolean;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndByte")]
    pub type AndByte = crate::GlobalNamespace::AndInstruction_AndByte;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndInt16")]
    pub type AndInt16 = crate::GlobalNamespace::AndInstruction_AndInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndInt32")]
    pub type AndInt32 = crate::GlobalNamespace::AndInstruction_AndInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndInt64")]
    pub type AndInt64 = crate::GlobalNamespace::AndInstruction_AndInt64;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndSByte")]
    pub type AndSByte = crate::GlobalNamespace::AndInstruction_AndSByte;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndUInt16")]
    pub type AndUInt16 = crate::GlobalNamespace::AndInstruction_AndUInt16;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndUInt32")]
    pub type AndUInt32 = crate::GlobalNamespace::AndInstruction_AndUInt32;
    #[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction+AndUInt64")]
    pub type AndUInt64 = crate::GlobalNamespace::AndInstruction_AndUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+AndInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::AndInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
