#[cfg(feature = "System+Linq+Expressions+InvocationExpression4")]
#[repr(C)]
#[derive(Debug)]
pub struct InvocationExpression4 {
    __cordl_parent: crate::System::Linq::Expressions::InvocationExpression,
    pub _arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    pub _arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    pub _arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression4")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::InvocationExpression4 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "InvocationExpression4";
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
#[cfg(feature = "System+Linq+Expressions+InvocationExpression4")]
impl std::ops::Deref for crate::System::Linq::Expressions::InvocationExpression4 {
    type Target = crate::System::Linq::Expressions::InvocationExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression4")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::InvocationExpression4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression4")]
impl crate::System::Linq::Expressions::InvocationExpression4 {
    pub fn GetArgument(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Linq::Expressions::InvocationExpression4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                1usize,
            >("GetArgument")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Linq::Expressions::InvocationExpression4 as
                    quest_hook::libil2cpp::Type > ::class(), "GetArgument", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        lambda: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        returnType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lambda, returnType, arg0, arg1, arg2, arg3))?;
        Ok(__cordl_object.into())
    }
    pub fn Rewrite(
        &mut self,
        lambda: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Linq::Expressions::InvocationExpression4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Linq::Expressions::Expression,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::InvocationExpression,
                >,
                2usize,
            >("Rewrite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Linq::Expressions::InvocationExpression4 as
                    quest_hook::libil2cpp::Type > ::class(), "Rewrite", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::InvocationExpression,
        > = unsafe { method.invoke_unchecked(self, (lambda, arguments))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        lambda: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        returnType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Linq::Expressions::InvocationExpression4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Linq::Expressions::InvocationExpression4 as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (lambda, returnType, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ArgumentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Linq::Expressions::InvocationExpression4 as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_ArgumentCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Linq::Expressions::InvocationExpression4 as
                    quest_hook::libil2cpp::Type > ::class(), "get_ArgumentCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+InvocationExpression4")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::InvocationExpression4 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
