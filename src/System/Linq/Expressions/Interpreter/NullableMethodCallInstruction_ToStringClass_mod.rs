#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+ToStringClass"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NullableMethodCallInstruction_ToStringClass {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::NullableMethodCallInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+ToStringClass"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NullableMethodCallInstruction_ToStringClass =>
    "System.Linq.Expressions.Interpreter"."NullableMethodCallInstruction/ToStringClass"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+ToStringClass"
)]
impl std::ops::Deref
for crate::GlobalNamespace::NullableMethodCallInstruction_ToStringClass {
    type Target = crate::System::Linq::Expressions::Interpreter::NullableMethodCallInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+ToStringClass"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::NullableMethodCallInstruction_ToStringClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+ToStringClass"
)]
impl crate::GlobalNamespace::NullableMethodCallInstruction_ToStringClass {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NullableMethodCallInstruction+ToStringClass"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NullableMethodCallInstruction_ToStringClass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
