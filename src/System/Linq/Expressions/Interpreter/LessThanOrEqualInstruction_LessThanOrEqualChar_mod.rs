#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualChar"
)]
#[repr(C)]
#[derive(Debug)]
pub struct LessThanOrEqualInstruction_LessThanOrEqualChar {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualChar"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "LessThanOrEqualInstruction/LessThanOrEqualChar";
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
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualChar"
)]
impl std::ops::Deref
for crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar {
    type Target = crate::System::Linq::Expressions::Interpreter::LessThanOrEqualInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualChar"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualChar"
)]
impl crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar {
    pub fn New(
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nullValue))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
                >),
                i32,
                1usize,
            >("Run")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar as
                    quest_hook::libil2cpp::Type > ::class(), "Run", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (frame))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nullValue))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanOrEqualInstruction+LessThanOrEqualChar"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LessThanOrEqualInstruction_LessThanOrEqualChar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
