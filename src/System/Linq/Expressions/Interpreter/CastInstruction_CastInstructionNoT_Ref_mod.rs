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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::CastInstructionNoT_CastInstruction_Ref {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "CastInstruction/CastInstructionNoT/Ref";
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
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ConvertNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConvertNull", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (frame))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (t))
        };
        Ok(__cordl_ret.into())
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
