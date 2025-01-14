#[cfg(feature = "System+Linq+Expressions+ExpressionN_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionN_1<TDelegate: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Linq::Expressions::Expression_1<TDelegate>,
    pub _parameters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    >,
    __cordl_phantom_TDelegate: std::marker::PhantomData<TDelegate>,
}
#[cfg(feature = "System+Linq+Expressions+ExpressionN_1")]
unsafe impl<TDelegate: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::ExpressionN_1<TDelegate> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "ExpressionN`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq.Expressions",
                        "ExpressionN`1",
                    )
                    .unwrap()
                    .make_generic::<(TDelegate)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "System+Linq+Expressions+ExpressionN_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Expressions::ExpressionN_1<TDelegate> {
    type Target = crate::System::Linq::Expressions::Expression_1<TDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionN_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Expressions::ExpressionN_1<TDelegate> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionN_1")]
impl<
    TDelegate: quest_hook::libil2cpp::Type,
> crate::System::Linq::Expressions::ExpressionN_1<TDelegate> {
    pub fn GetParameter(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ParameterExpression>,
    >
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
                1usize,
            >("GetParameter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetParameter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ParameterExpression,
        > = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (body, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn Rewrite(
        &mut self,
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<TDelegate>,
        >,
    >
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Linq::Expressions::ParameterExpression,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::Expression_1<TDelegate>,
                >,
                2usize,
            >("Rewrite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Rewrite", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression_1<TDelegate>,
        > = unsafe { method.invoke_unchecked(self, (body, parameters)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IReadOnlyList_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Linq::Expressions::ParameterExpression,
                            >,
                        >,
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
            method.invoke_unchecked(self, (body, parameters))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ParameterCount(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_ParameterCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ParameterCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionN_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ExpressionN_1<TDelegate> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
