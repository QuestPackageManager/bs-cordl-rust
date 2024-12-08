#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct ByRefUpdater {
    __cordl_parent: crate::System::Object,
    pub ArgumentIndex: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::ByRefUpdater =>
    "System.Linq.Expressions.Interpreter"."ByRefUpdater"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefUpdater")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::ByRefUpdater {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefUpdater")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Interpreter::ByRefUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefUpdater")]
impl crate::System::Linq::Expressions::Interpreter::ByRefUpdater {
    pub fn Update(
        &mut self,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (frame, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (argumentIndex))?;
        Ok(__cordl_ret)
    }
    pub fn UndefineTemps(
        &mut self,
        instructions: *mut crate::System::Linq::Expressions::Interpreter::InstructionList,
        locals: *mut crate::System::Linq::Expressions::Interpreter::LocalVariables,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UndefineTemps", (instructions, locals))?;
        Ok(__cordl_ret)
    }
    pub fn New(argumentIndex: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (argumentIndex))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ByRefUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ByRefUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
