#[cfg(feature = "System+Linq+Expressions+Interpreter+ParameterByRefUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct ParameterByRefUpdater {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
    pub _parameter: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ParameterByRefUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::ParameterByRefUpdater =>
    "System.Linq.Expressions.Interpreter"."ParameterByRefUpdater"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+ParameterByRefUpdater")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::ParameterByRefUpdater {
    type Target = crate::System::Linq::Expressions::Interpreter::ByRefUpdater;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ParameterByRefUpdater")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::ParameterByRefUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ParameterByRefUpdater")]
impl crate::System::Linq::Expressions::Interpreter::ParameterByRefUpdater {
    pub fn _ctor(
        &mut self,
        parameter: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameter, argumentIndex))?;
        Ok(__cordl_ret)
    }
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
    pub fn New(
        parameter: *mut crate::System::Linq::Expressions::Interpreter::LocalVariable,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameter, argumentIndex))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ParameterByRefUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ParameterByRefUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
