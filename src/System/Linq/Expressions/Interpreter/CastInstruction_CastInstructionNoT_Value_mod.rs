#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Value"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CastInstructionNoT_CastInstruction_Value {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CastInstruction_CastInstructionNoT,
    >,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Value"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CastInstructionNoT_CastInstruction_Value =>
    "System.Linq.Expressions.Interpreter"."CastInstruction/CastInstructionNoT/Value"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Value"
)]
impl std::ops::Deref
for crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Value {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CastInstruction_CastInstructionNoT,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Value"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Value"
)]
impl crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Value {
    pub fn ConvertNull(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertNull", (frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (t))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Value"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Value {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
