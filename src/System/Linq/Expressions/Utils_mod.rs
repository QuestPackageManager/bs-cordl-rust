#[cfg(feature = "System+Linq+Expressions+Utils")]
#[repr(C)]
#[derive(Debug)]
pub struct Utils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Utils")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Linq::Expressions::Utils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "Utils";
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
#[cfg(feature = "System+Linq+Expressions+Utils")]
impl std::ops::Deref for crate::System::Linq::Expressions::Utils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Utils")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Utils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Utils")]
impl crate::System::Linq::Expressions::Utils {
    pub fn Constant__cordl_bool0(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ConstantExpression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ConstantExpression,
                >,
                1usize,
            >("Constant")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Constant", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConstantExpression,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn Constant_i32_1(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ConstantExpression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ConstantExpression,
                >,
                1usize,
            >("Constant")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Constant", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConstantExpression,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Utils")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::Utils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
