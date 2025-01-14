#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
#[repr(C)]
#[derive(Debug)]
pub struct LightDelegateCreator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _lambda: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::LambdaExpression,
    >,
    pub _Interpreter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::Interpreter,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "LightDelegateCreator";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
impl crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    pub fn CreateDelegate_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                0usize,
            >("CreateDelegate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateDelegate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_Il2CppArray1(
        &mut self,
        closure: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::CompilerServices::IStrongBox,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::CompilerServices::IStrongBox,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                1usize,
            >("CreateDelegate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateDelegate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = unsafe {
            method.invoke_unchecked(self, (closure))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        interpreter: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Interpreter,
        >,
        lambda: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (interpreter, lambda))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        interpreter: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Interpreter,
        >,
        lambda: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LambdaExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Interpreter::Interpreter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::LambdaExpression,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (interpreter, lambda))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Interpreter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Interpreter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Interpreter::Interpreter,
                >,
                0usize,
            >("get_Interpreter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Interpreter", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::Interpreter,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LightDelegateCreator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LightDelegateCreator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
