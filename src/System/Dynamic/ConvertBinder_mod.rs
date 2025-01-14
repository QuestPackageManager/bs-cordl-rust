#[cfg(feature = "System+Dynamic+ConvertBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConvertBinder {
    __cordl_parent: crate::System::Dynamic::DynamicMetaObjectBinder,
    pub _Type_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+Dynamic+ConvertBinder")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Dynamic::ConvertBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Dynamic";
    const CLASS_NAME: &'static str = "ConvertBinder";
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
#[cfg(feature = "System+Dynamic+ConvertBinder")]
impl std::ops::Deref for crate::System::Dynamic::ConvertBinder {
    type Target = crate::System::Dynamic::DynamicMetaObjectBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ConvertBinder")]
impl std::ops::DerefMut for crate::System::Dynamic::ConvertBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ConvertBinder")]
impl crate::System::Dynamic::ConvertBinder {
    pub fn Bind(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Dynamic::DynamicMetaObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
                2usize,
            >("Bind")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Bind", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = unsafe { method.invoke_unchecked(self, (target, args)) };
        Ok(__cordl_ret.into())
    }
    pub fn FallbackConvert_DynamicMetaObject0(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>),
                quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
                1usize,
            >("FallbackConvert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FallbackConvert", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = unsafe { method.invoke_unchecked(self, (target)) };
        Ok(__cordl_ret.into())
    }
    pub fn FallbackConvert_DynamicMetaObject1(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        errorSuggestion: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
                2usize,
            >("FallbackConvert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FallbackConvert", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = unsafe { method.invoke_unchecked(self, (target, errorSuggestion)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                0usize,
            >("get_Type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Type", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+ConvertBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::ConvertBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
