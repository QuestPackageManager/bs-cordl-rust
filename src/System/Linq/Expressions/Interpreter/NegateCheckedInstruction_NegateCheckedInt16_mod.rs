#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt16"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NegateCheckedInstruction_NegateCheckedInt16 {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::NegateCheckedInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt16"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NegateCheckedInstruction_NegateCheckedInt16 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "NegateCheckedInstruction/NegateCheckedInt16";
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
    feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt16"
)]
impl std::ops::Deref
for crate::GlobalNamespace::NegateCheckedInstruction_NegateCheckedInt16 {
    type Target = crate::System::Linq::Expressions::Interpreter::NegateCheckedInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt16"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::NegateCheckedInstruction_NegateCheckedInt16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt16"
)]
impl crate::GlobalNamespace::NegateCheckedInstruction_NegateCheckedInt16 {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
                >),
                i32,
                1usize,
            >("Run")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Run", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (frame)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NegateCheckedInstruction+NegateCheckedInt16"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NegateCheckedInstruction_NegateCheckedInt16 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
