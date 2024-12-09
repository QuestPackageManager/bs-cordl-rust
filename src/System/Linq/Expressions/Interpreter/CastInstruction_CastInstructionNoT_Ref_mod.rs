#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Ref"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CastInstructionNoT_CastInstruction_Ref {
    __cordl_parent: crate::GlobalNamespace::CastInstruction_CastInstructionNoT,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Ref"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CastInstructionNoT_CastInstruction_Ref =>
    "System.Linq.Expressions.Interpreter"."CastInstruction/CastInstructionNoT/Ref"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Ref"
)]
impl std::ops::Deref for crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Ref {
    type Target = crate::GlobalNamespace::CastInstruction_CastInstructionNoT;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Ref"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Ref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Ref"
)]
impl crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Ref {
    pub fn ConvertNull(
        &mut self,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertNull", (frame))?;
        Ok(__cordl_ret)
    }
    pub fn New(t: *mut crate::System::Type) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (t))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (t))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Ref"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Ref {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
