#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncInstallerRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub monoInstallers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Zenject::MonoInstaller>,
        >,
    >,
    pub scriptableObjectInstallers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Zenject::ScriptableObjectInstaller>,
        >,
    >,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.AppFlow.Initialization";
    const CLASS_NAME: &'static str = "AsyncInstallerRegistry";
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
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl std::ops::Deref for crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl std::ops::DerefMut
for crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    pub fn AddMonoInstaller(
        &mut self,
        newMonoInstaller: quest_hook::libil2cpp::Gc<crate::Zenject::MonoInstaller>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::MonoInstaller>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddMonoInstaller")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddMonoInstaller", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newMonoInstaller))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddScriptableObjectInstaller(
        &mut self,
        newScriptableObjectInstaller: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScriptableObjectInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Zenject::ScriptableObjectInstaller,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddScriptableObjectInstaller")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddScriptableObjectInstaller", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newScriptableObjectInstaller))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl AsRef<crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry>
for crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    fn as_ref(
        &self,
    ) -> &crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl AsMut<crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry>
for crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    fn as_mut(
        &mut self,
    ) -> &mut crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry {
        unsafe { std::mem::transmute(self) }
    }
}
