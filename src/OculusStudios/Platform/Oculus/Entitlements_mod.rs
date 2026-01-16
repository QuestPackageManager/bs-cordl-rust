#[cfg(feature = "cordl_class_OculusStudios+Platform+Oculus+Entitlements")]
#[repr(C)]
#[derive(Debug)]
pub struct Entitlements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub logger: quest_hook::libil2cpp::Gc<
        crate::OculusStudios::Platform::Core::IPlatformLogger,
    >,
    pub productDefinitions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::OculusStudios::Platform::Core::IProductDefinition,
            >,
        >,
    >,
    pub productInstances: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::OculusStudios::Platform::Core::Product>,
        >,
    >,
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Oculus+Entitlements")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::Platform::Oculus::Entitlements {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.Platform.Oculus";
    const CLASS_NAME: &'static str = "Entitlements";
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
#[cfg(feature = "OculusStudios+Platform+Oculus+Entitlements")]
impl std::ops::Deref for crate::OculusStudios::Platform::Oculus::Entitlements {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Oculus+Entitlements")]
impl std::ops::DerefMut for crate::OculusStudios::Platform::Oculus::Entitlements {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Oculus+Entitlements")]
impl crate::OculusStudios::Platform::Oculus::Entitlements {
    pub fn GetAllProductsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::Platform::Core::Product,
                        >,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::IReadOnlyList_1<
                                        quest_hook::libil2cpp::Gc<
                                            crate::OculusStudios::Platform::Core::Product,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        0usize,
                    >("GetAllProductsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllProductsAsync", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::Platform::Core::Product,
                        >,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initParams: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::Platform::Core::PlatformInitParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initParams))?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterNewProducts(
        &mut self,
        productDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::Platform::Core::IProductDefinition,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::OculusStudios::Platform::Core::IProductDefinition,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RegisterNewProducts")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterNewProducts", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (productDefinitions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initParams: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::Platform::Core::PlatformInitParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::Platform::Core::PlatformInitParams,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (initParams))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Oculus+Entitlements")]
impl quest_hook::libil2cpp::ObjectType
for crate::OculusStudios::Platform::Oculus::Entitlements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusStudios+Platform+Oculus+Entitlements")]
impl AsRef<crate::OculusStudios::Platform::Core::IPlatformEntitlements>
for crate::OculusStudios::Platform::Oculus::Entitlements {
    fn as_ref(&self) -> &crate::OculusStudios::Platform::Core::IPlatformEntitlements {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusStudios+Platform+Oculus+Entitlements")]
impl AsMut<crate::OculusStudios::Platform::Core::IPlatformEntitlements>
for crate::OculusStudios::Platform::Oculus::Entitlements {
    fn as_mut(
        &mut self,
    ) -> &mut crate::OculusStudios::Platform::Core::IPlatformEntitlements {
        unsafe { std::mem::transmute(self) }
    }
}
