#[cfg(feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct RightShiftInstruction {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::Instruction,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::RightShiftInstruction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "RightShiftInstruction";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::RightShiftInstruction {
    type Target = crate::System::Linq::Expressions::Interpreter::Instruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::RightShiftInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction")]
impl crate::System::Linq::Expressions::Interpreter::RightShiftInstruction {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftByte"
    )]
    pub type RightShiftByte = crate::GlobalNamespace::RightShiftInstruction_RightShiftByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftInt16"
    )]
    pub type RightShiftInt16 = crate::GlobalNamespace::RightShiftInstruction_RightShiftInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftInt32"
    )]
    pub type RightShiftInt32 = crate::GlobalNamespace::RightShiftInstruction_RightShiftInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftInt64"
    )]
    pub type RightShiftInt64 = crate::GlobalNamespace::RightShiftInstruction_RightShiftInt64;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftSByte"
    )]
    pub type RightShiftSByte = crate::GlobalNamespace::RightShiftInstruction_RightShiftSByte;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftUInt16"
    )]
    pub type RightShiftUInt16 = crate::GlobalNamespace::RightShiftInstruction_RightShiftUInt16;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftUInt32"
    )]
    pub type RightShiftUInt32 = crate::GlobalNamespace::RightShiftInstruction_RightShiftUInt32;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction+RightShiftUInt64"
    )]
    pub type RightShiftUInt64 = crate::GlobalNamespace::RightShiftInstruction_RightShiftUInt64;
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+RightShiftInstruction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::RightShiftInstruction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
