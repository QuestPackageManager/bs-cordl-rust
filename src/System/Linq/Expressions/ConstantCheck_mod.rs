#[cfg(feature = "cordl_class_System+Linq+Expressions+ConstantCheck")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstantCheck {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+ConstantCheck")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::ConstantCheck {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "ConstantCheck";
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
#[cfg(feature = "cordl_class_System+Linq+Expressions+ConstantCheck")]
impl std::ops::Deref for crate::System::Linq::Expressions::ConstantCheck {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+ConstantCheck")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ConstantCheck {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl crate::System::Linq::Expressions::ConstantCheck {
    pub fn AnalyzeTypeIs_Expression_Type1(
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        testType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::AnalyzeTypeIsResult,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Linq::Expressions::Expression,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        crate::System::Linq::Expressions::AnalyzeTypeIsResult,
                        2usize,
                    >("AnalyzeTypeIs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AnalyzeTypeIs", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Linq::Expressions::AnalyzeTypeIsResult = unsafe {
            cordl_method_info.invoke_unchecked((), (operand, testType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AnalyzeTypeIs_TypeBinaryExpression0(
        typeIs: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::TypeBinaryExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::AnalyzeTypeIsResult,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::TypeBinaryExpression,
                        >),
                        crate::System::Linq::Expressions::AnalyzeTypeIsResult,
                        1usize,
                    >("AnalyzeTypeIs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AnalyzeTypeIs", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Linq::Expressions::AnalyzeTypeIsResult = unsafe {
            cordl_method_info.invoke_unchecked((), (typeIs))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+ConstantCheck")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ConstantCheck {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
