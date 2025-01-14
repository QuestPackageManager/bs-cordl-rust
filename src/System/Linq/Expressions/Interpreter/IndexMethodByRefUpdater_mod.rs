#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexMethodByRefUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexMethodByRefUpdater {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::ByRefUpdater,
    pub _indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub _obj: crate::System::Nullable_1<
        crate::System::Linq::Expressions::Interpreter::LocalDefinition,
    >,
    pub _args: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexMethodByRefUpdater")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::IndexMethodByRefUpdater {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "IndexMethodByRefUpdater";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexMethodByRefUpdater")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::IndexMethodByRefUpdater {
    type Target = crate::System::Linq::Expressions::Interpreter::ByRefUpdater;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexMethodByRefUpdater")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::IndexMethodByRefUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexMethodByRefUpdater")]
impl crate::System::Linq::Expressions::Interpreter::IndexMethodByRefUpdater {
    pub fn New(
        obj: crate::System::Nullable_1<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Linq::Expressions::Interpreter::LocalDefinition,
            >,
        >,
        indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obj, args, indexer, argumentIndex))?;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Interpreter::InstructionList,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Interpreter::LocalVariables,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UndefineTemps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UndefineTemps", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (instructions, locals))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (frame, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        obj: crate::System::Nullable_1<
            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Linq::Expressions::Interpreter::LocalDefinition,
            >,
        >,
        indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        argumentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Nullable_1<
                        crate::System::Linq::Expressions::Interpreter::LocalDefinition,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::System::Linq::Expressions::Interpreter::LocalDefinition,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obj, args, indexer, argumentIndex))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+IndexMethodByRefUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::IndexMethodByRefUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
