#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct ProviderBindingFinalizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _BindInfo_k__BackingField: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::ProviderBindingFinalizer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ProviderBindingFinalizer";
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
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::ProviderBindingFinalizer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::ProviderBindingFinalizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl crate::Zenject::ProviderBindingFinalizer {
    pub fn FinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FinalizeBinding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FinalizeBinding", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::ScopeTypes> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::Zenject::ScopeTypes, 0usize>("GetScope")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetScope", 0usize
                )
            });
        let __cordl_ret: crate::Zenject::ScopeTypes = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn OnFinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnFinalizeBinding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnFinalizeBinding", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProviderForAllContracts(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RegisterProviderForAllContracts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterProviderForAllContracts", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProviderPerContract(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        providerFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_3<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RegisterProviderPerContract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterProviderPerContract", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, providerFunc))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProvider_IProvider0<TContract>(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RegisterProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterProvider", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProvider_Type_IProvider1(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        contractType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RegisterProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterProvider", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, contractType, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProvidersForAllContractsPerConcreteType(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        providerFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_3<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RegisterProvidersForAllContractsPerConcreteType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterProvidersForAllContractsPerConcreteType", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, concreteTypes, providerFunc))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProvidersPerContractAndConcreteType(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        providerFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_3<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RegisterProvidersPerContractAndConcreteType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterProvidersPerContractAndConcreteType", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, concreteTypes, providerFunc))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateBindTypes(
        &mut self,
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        contractType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                bool,
                2usize,
            >("ValidateBindTypes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ValidateBindTypes", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (concreteType, contractType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bindInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
                0usize,
            >("get_BindInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_BindInfo", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BindingInheritanceMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::BindingInheritanceMethods> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::Zenject::BindingInheritanceMethods,
                0usize,
            >("get_BindingInheritanceMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_BindingInheritanceMethod", 0usize
                )
            });
        let __cordl_ret: crate::Zenject::BindingInheritanceMethods = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_BindInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_BindInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_BindInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ProviderBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl AsRef<crate::Zenject::IBindingFinalizer>
for crate::Zenject::ProviderBindingFinalizer {
    fn as_ref(&self) -> &crate::Zenject::IBindingFinalizer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl AsMut<crate::Zenject::IBindingFinalizer>
for crate::Zenject::ProviderBindingFinalizer {
    fn as_mut(&mut self) -> &mut crate::Zenject::IBindingFinalizer {
        unsafe { std::mem::transmute(self) }
    }
}
