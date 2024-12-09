#[cfg(feature = "System+Linq+Expressions+Interpreter+FieldByRefUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct FieldByRefUpdater {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
    pub _object: crate::System::Nullable_1<
        crate::System::Linq::Expressions::Interpreter::LocalDefinition,
    >,
    pub _field: *mut crate::System::Reflection::FieldInfo,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+FieldByRefUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::FieldByRefUpdater =>
    "System.Linq.Expressions.Interpreter"."FieldByRefUpdater"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+FieldByRefUpdater")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::FieldByRefUpdater {
    type Target = crate::System::Linq::Expressions::Interpreter::ByRefUpdater;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+FieldByRefUpdater")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::FieldByRefUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+FieldByRefUpdater")]
impl crate::System::Linq::Expressions::Interpreter::FieldByRefUpdater {
    pub fn New(
        obj: crate::System::Nullable_1<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
        field: *mut crate::System::Reflection::FieldInfo,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obj, field, argumentIndex))?;
        Ok(__cordl_object)
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
    pub fn Update(
        &mut self,
        frame: *mut crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
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
        obj: crate::System::Nullable_1<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
        field: *mut crate::System::Reflection::FieldInfo,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (obj, field, argumentIndex))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+FieldByRefUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::FieldByRefUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
