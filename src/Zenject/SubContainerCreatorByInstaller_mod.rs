#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByInstaller {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _extraArgs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
    >,
    pub _containerBindInfo: quest_hook::libil2cpp::Gc<
        crate::Zenject::SubContainerCreatorBindInfo,
    >,
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::SubContainerCreatorByInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "SubContainerCreatorByInstaller";
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
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByInstaller {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByInstaller {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl crate::Zenject::SubContainerCreatorByInstaller {
    pub fn CreateSubContainer(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::Zenject::TypeValuePair,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        2usize,
                    >("CreateSubContainer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSubContainer", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = unsafe {
            cordl_method_info.invoke_unchecked(self, (args, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_DiContainer_SubContainerCreatorBindInfo_Type1(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        containerBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SubContainerCreatorBindInfo,
        >,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, containerBindInfo, installerType))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEnumerable_1_0(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        containerBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SubContainerCreatorBindInfo,
        >,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (container, containerBindInfo, installerType, extraArgs),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_DiContainer_SubContainerCreatorBindInfo_Type1(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        containerBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SubContainerCreatorBindInfo,
        >,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::SubContainerCreatorBindInfo,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
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
            cordl_method_info
                .invoke_unchecked(self, (container, containerBindInfo, installerType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_0(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        containerBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SubContainerCreatorBindInfo,
        >,
        installerType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::SubContainerCreatorBindInfo,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<
                                    crate::Zenject::TypeValuePair,
                                >,
                            >,
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
                    (container, containerBindInfo, installerType, extraArgs),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl AsRef<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorByInstaller {
    fn as_ref(&self) -> &crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl AsMut<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorByInstaller {
    fn as_mut(&mut self) -> &mut crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
