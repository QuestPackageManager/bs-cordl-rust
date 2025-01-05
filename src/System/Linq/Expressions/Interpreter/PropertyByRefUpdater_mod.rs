#[cfg(feature = "System+Linq+Expressions+Interpreter+PropertyByRefUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyByRefUpdater {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
    >,
    pub _object: crate::System::Nullable_1<
        crate::System::Linq::Expressions::Interpreter::LocalDefinition,
    >,
    pub _property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+PropertyByRefUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::PropertyByRefUpdater =>
    "System.Linq.Expressions.Interpreter"."PropertyByRefUpdater"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+PropertyByRefUpdater")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::PropertyByRefUpdater {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+PropertyByRefUpdater")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::PropertyByRefUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+PropertyByRefUpdater")]
impl crate::System::Linq::Expressions::Interpreter::PropertyByRefUpdater {
    pub fn New(
        obj: crate::System::Nullable_1<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obj, property, argumentIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn UndefineTemps(
        &mut self,
        instructions: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InstructionList,
        >,
        locals: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LocalVariables,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UndefineTemps", (instructions, locals))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (frame, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        obj: crate::System::Nullable_1<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (obj, property, argumentIndex))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+PropertyByRefUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::PropertyByRefUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
