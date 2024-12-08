#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CastInstruction_CastInstructionNoT {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::CastInstruction,
    pub _t: *mut crate::System::Type,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CastInstruction_CastInstructionNoT =>
    "System.Linq.Expressions.Interpreter"."CastInstruction/CastInstructionNoT"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT"
)]
impl std::ops::Deref for crate::GlobalNamespace::CastInstruction_CastInstructionNoT {
    type Target = crate::System::Linq::Expressions::Interpreter::CastInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT"
)]
impl std::ops::DerefMut for crate::GlobalNamespace::CastInstruction_CastInstructionNoT {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT"
)]
impl crate::GlobalNamespace::CastInstruction_CastInstructionNoT {
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Ref"
    )]
    pub type Ref = crate::GlobalNamespace::CastInstructionNoT_Ref;
    #[cfg(
        feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT+Value"
    )]
    pub type Value = crate::GlobalNamespace::CastInstructionNoT_Value;
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
    pub fn Run(
        &mut self,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret)
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
    feature = "System+Linq+Expressions+Interpreter+CastInstruction+CastInstructionNoT"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CastInstruction_CastInstructionNoT {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
