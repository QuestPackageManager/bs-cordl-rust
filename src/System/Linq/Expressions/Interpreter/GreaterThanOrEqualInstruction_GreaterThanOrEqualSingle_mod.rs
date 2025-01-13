#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSingle"
)]
#[repr(C)]
#[derive(Debug)]
pub struct GreaterThanOrEqualInstruction_GreaterThanOrEqualSingle {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSingle"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSingle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "GreaterThanOrEqualInstruction/GreaterThanOrEqualSingle";
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
    feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSingle"
)]
impl std::ops::Deref
for crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSingle {
    type Target = crate::System::Linq::Expressions::Interpreter::GreaterThanOrEqualInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSingle"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSingle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSingle"
)]
impl crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSingle {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret.into())
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
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+GreaterThanOrEqualInstruction+GreaterThanOrEqualSingle"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GreaterThanOrEqualInstruction_GreaterThanOrEqualSingle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
