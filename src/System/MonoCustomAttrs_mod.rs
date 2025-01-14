#[cfg(feature = "System+MonoCustomAttrs")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoCustomAttrs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+MonoCustomAttrs")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::MonoCustomAttrs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "MonoCustomAttrs";
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
#[cfg(feature = "System+MonoCustomAttrs")]
impl std::ops::Deref for crate::System::MonoCustomAttrs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl std::ops::DerefMut for crate::System::MonoCustomAttrs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl crate::System::MonoCustomAttrs {
    #[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
    pub type AttributeInfo = crate::System::MonoCustomAttrs_AttributeInfo;
    pub fn GetBase(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ICustomAttributeProvider>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Reflection::ICustomAttributeProvider,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Reflection::ICustomAttributeProvider,
                >,
                1usize,
            >("GetBase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBase", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseEventDefinition(
        evt: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeEventInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeEventInfo>),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
                1usize,
            >("GetBaseEventDefinition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBaseEventDefinition", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::EventInfo,
        > = unsafe { method.invoke_unchecked((), (evt)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetBasePropertyDefinition(
        property: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::RuntimePropertyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Reflection::RuntimePropertyInfo,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
                1usize,
            >("GetBasePropertyDefinition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBasePropertyDefinition", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::PropertyInfo,
        > = unsafe { method.invoke_unchecked((), (property)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesBase(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inheritedOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                3usize,
            >("GetCustomAttributesBase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCustomAttributesBase", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, attributeType, inheritedOnly)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesDataBase(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inheritedOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::CustomAttributeData,
                        >,
                    >,
                >,
                3usize,
            >("GetCustomAttributesDataBase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCustomAttributesDataBase", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, attributeType, inheritedOnly)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesDataInternal(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Reflection::ICustomAttributeProvider,
                >),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::CustomAttributeData,
                        >,
                    >,
                >,
                1usize,
            >("GetCustomAttributesDataInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCustomAttributesDataInternal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesData_Type__cordl_bool1(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::CustomAttributeData,
                        >,
                    >,
                >,
                3usize,
            >("GetCustomAttributesData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCustomAttributesData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, attributeType, inherit)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesData__cordl_bool0(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::CustomAttributeData,
                        >,
                    >,
                >,
                2usize,
            >("GetCustomAttributesData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCustomAttributesData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, inherit)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributesInternal(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        pseudoAttrs: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                    >,
                >,
                3usize,
            >("GetCustomAttributesInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCustomAttributesInternal", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, attributeType, pseudoAttrs)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Type__cordl_bool0(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                3usize,
            >("GetCustomAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCustomAttributes", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, attributeType, inherit)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes__cordl_bool1(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                2usize,
            >("GetCustomAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCustomAttributes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, inherit)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributesData_ICustomAttributeProvider_Type0(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::CustomAttributeData,
                        >,
                    >,
                >,
                2usize,
            >("GetPseudoCustomAttributesData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPseudoCustomAttributesData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, attributeType)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributesData_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Reflection::CustomAttributeData,
                        >,
                    >,
                >,
                1usize,
            >("GetPseudoCustomAttributesData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPseudoCustomAttributesData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::CustomAttributeData>,
            >,
        > = unsafe { method.invoke_unchecked((), (_cordl_type)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributes_ICustomAttributeProvider_Type0(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                2usize,
            >("GetPseudoCustomAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPseudoCustomAttributes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked((), (obj, attributeType)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPseudoCustomAttributes_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                1usize,
            >("GetPseudoCustomAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPseudoCustomAttributes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked((), (_cordl_type)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    bool,
                ),
                bool,
                3usize,
            >("IsDefined")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsDefined", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (obj, attributeType, inherit))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDefinedInternal(
        obj: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
        AttributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::ICustomAttributeProvider,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                bool,
                2usize,
            >("IsDefinedInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsDefinedInternal", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (obj, AttributeType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsUserCattrProvider(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("IsUserCattrProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsUserCattrProvider", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn RetrieveAttributeUsage(
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
                1usize,
            >("RetrieveAttributeUsage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RetrieveAttributeUsage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::AttributeUsageAttribute,
        > = unsafe { method.invoke_unchecked((), (attributeType)) };
        Ok(__cordl_ret.into())
    }
    pub fn RetrieveAttributeUsageNoCache(
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
                1usize,
            >("RetrieveAttributeUsageNoCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RetrieveAttributeUsageNoCache", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::AttributeUsageAttribute,
        > = unsafe { method.invoke_unchecked((), (attributeType)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+MonoCustomAttrs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoCustomAttrs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoCustomAttrs_AttributeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _usage: quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
    pub _inheritanceLevel: i32,
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::MonoCustomAttrs_AttributeInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "MonoCustomAttrs/AttributeInfo";
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
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl std::ops::Deref for crate::System::MonoCustomAttrs_AttributeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl std::ops::DerefMut for crate::System::MonoCustomAttrs_AttributeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl crate::System::MonoCustomAttrs_AttributeInfo {
    pub fn New(
        usage: quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
        inheritanceLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (usage, inheritanceLevel))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        usage: quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
        inheritanceLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>, i32),
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
            method.invoke_unchecked(self, (usage, inheritanceLevel))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_InheritanceLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_InheritanceLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_InheritanceLevel", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Usage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::AttributeUsageAttribute>,
                0usize,
            >("get_Usage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Usage", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::AttributeUsageAttribute,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+MonoCustomAttrs+AttributeInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoCustomAttrs_AttributeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
