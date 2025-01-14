#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct ListInitExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _NewExpression_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::NewExpression,
    >,
    pub _Initializers_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ElementInit>,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::ListInitExpression {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "ListInitExpression";
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
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::ListInitExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ListInitExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
impl crate::System::Linq::Expressions::ListInitExpression {
    pub fn get_Initializers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ElementInit>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::ElementInit,
                        >,
                    >,
                >,
                0usize,
            >("get_Initializers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Initializers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ElementInit>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_NewExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::NewExpression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::NewExpression,
                >,
                0usize,
            >("get_NewExpression")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_NewExpression", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::NewExpression,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+ListInitExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ListInitExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
