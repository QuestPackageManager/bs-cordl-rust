#[cfg(feature = "System+Linq+Expressions+Interpreter+TryFaultHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TryFaultHandler {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub TryStartIndex: i32,
    pub TryEndIndex: i32,
    pub FinallyStartIndex: i32,
    pub FinallyEndIndex: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryFaultHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::TryFaultHandler =>
    "System.Linq.Expressions.Interpreter"."TryFaultHandler"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryFaultHandler")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::TryFaultHandler {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryFaultHandler")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::TryFaultHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryFaultHandler")]
impl crate::System::Linq::Expressions::Interpreter::TryFaultHandler {
    pub fn New(
        tryStart: i32,
        tryEnd: i32,
        finallyStart: i32,
        finallyEnd: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tryStart, tryEnd, finallyStart, finallyEnd))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tryStart: i32,
        tryEnd: i32,
        finallyStart: i32,
        finallyEnd: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tryStart, tryEnd, finallyStart, finallyEnd))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+TryFaultHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::TryFaultHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
