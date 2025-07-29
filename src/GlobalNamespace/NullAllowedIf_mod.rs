#[cfg(feature = "cordl_class_NullAllowedIf")]
#[repr(C)]
#[derive(Debug)]
pub struct NullAllowedIf {
    __cordl_parent: crate::GlobalNamespace::NullAllowed,
    pub propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _valueToCompare: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _comparisonOperation: crate::GlobalNamespace::ComparisonOperation,
}
#[cfg(feature = "cordl_class_NullAllowedIf")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NullAllowedIf {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NullAllowedIf";
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
#[cfg(feature = "NullAllowedIf")]
impl std::ops::Deref for crate::GlobalNamespace::NullAllowedIf {
    type Target = crate::GlobalNamespace::NullAllowed;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowedIf")]
impl std::ops::DerefMut for crate::GlobalNamespace::NullAllowedIf {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowedIf")]
impl crate::GlobalNamespace::NullAllowedIf {
    pub fn IsNullAllowedFor(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::GlobalNamespace::NullAllowed_Context,
                        ),
                        bool,
                        2usize,
                    >("IsNullAllowedFor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsNullAllowedFor", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (value, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_ComparisonOperation_Il2CppObject_NullAllowed_Context1(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        comparisonOperation: crate::GlobalNamespace::ComparisonOperation,
        valueToCompare: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (propertyName, comparisonOperation, valueToCompare, context),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject_NullAllowed_Context0(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        equalsTo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName, equalsTo, context))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ComparisonOperation_Il2CppObject_NullAllowed_Context1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        comparisonOperation: crate::GlobalNamespace::ComparisonOperation,
        valueToCompare: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::ComparisonOperation,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::GlobalNamespace::NullAllowed_Context,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (propertyName, comparisonOperation, valueToCompare, context),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_NullAllowed_Context0(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        equalsTo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::GlobalNamespace::NullAllowed_Context,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (propertyName, equalsTo, context))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_NullAllowedIf")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NullAllowedIf {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
