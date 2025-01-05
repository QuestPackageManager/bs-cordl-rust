#[cfg(feature = "System+Linq+Expressions+Interpreter+ScriptingRuntimeHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptingRuntimeHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ScriptingRuntimeHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::ScriptingRuntimeHelpers =>
    "System.Linq.Expressions.Interpreter"."ScriptingRuntimeHelpers"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+ScriptingRuntimeHelpers")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::ScriptingRuntimeHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ScriptingRuntimeHelpers")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::ScriptingRuntimeHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ScriptingRuntimeHelpers")]
impl crate::System::Linq::Expressions::Interpreter::ScriptingRuntimeHelpers {
    pub fn GetPrimitiveDefaultValue(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrimitiveDefaultValue", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToObject(
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Int32ToObject", (i))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ScriptingRuntimeHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ScriptingRuntimeHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
