#[cfg(
    feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt32LiftedToNull"
)]
#[repr(C)]
#[derive(Debug)]
pub struct EqualInstruction_EqualUInt32LiftedToNull {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::EqualInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt32LiftedToNull"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EqualInstruction_EqualUInt32LiftedToNull =>
    "System.Linq.Expressions.Interpreter"."EqualInstruction/EqualUInt32LiftedToNull"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt32LiftedToNull"
)]
impl std::ops::Deref
for crate::GlobalNamespace::EqualInstruction_EqualUInt32LiftedToNull {
    type Target = crate::System::Linq::Expressions::Interpreter::EqualInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt32LiftedToNull"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::EqualInstruction_EqualUInt32LiftedToNull {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt32LiftedToNull"
)]
impl crate::GlobalNamespace::EqualInstruction_EqualUInt32LiftedToNull {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret.into())
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
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+EqualInstruction+EqualUInt32LiftedToNull"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EqualInstruction_EqualUInt32LiftedToNull {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
