#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct LessThanInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
    pub _nullValue: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanInstruction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LessThanInstruction =>
    "System.Linq.Expressions.Interpreter"."LessThanInstruction"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::LessThanInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LessThanInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanInstruction")]
impl crate::System::Linq::Expressions::Interpreter::LessThanInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanByte"
    )]
    pub type LessThanByte = crate::GlobalNamespace::LessThanInstruction_LessThanByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanChar"
    )]
    pub type LessThanChar = crate::GlobalNamespace::LessThanInstruction_LessThanChar;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanDouble"
    )]
    pub type LessThanDouble = crate::GlobalNamespace::LessThanInstruction_LessThanDouble;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanInt16"
    )]
    pub type LessThanInt16 = crate::GlobalNamespace::LessThanInstruction_LessThanInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanInt32"
    )]
    pub type LessThanInt32 = crate::GlobalNamespace::LessThanInstruction_LessThanInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanInt64"
    )]
    pub type LessThanInt64 = crate::GlobalNamespace::LessThanInstruction_LessThanInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanSByte"
    )]
    pub type LessThanSByte = crate::GlobalNamespace::LessThanInstruction_LessThanSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanSingle"
    )]
    pub type LessThanSingle = crate::GlobalNamespace::LessThanInstruction_LessThanSingle;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanUInt16"
    )]
    pub type LessThanUInt16 = crate::GlobalNamespace::LessThanInstruction_LessThanUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanUInt32"
    )]
    pub type LessThanUInt32 = crate::GlobalNamespace::LessThanInstruction_LessThanUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanUInt64"
    )]
    pub type LessThanUInt64 = crate::GlobalNamespace::LessThanInstruction_LessThanUInt64;
    pub fn Create(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        liftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Instruction,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (_cordl_type, liftedToNull))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LessThanInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LessThanInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
