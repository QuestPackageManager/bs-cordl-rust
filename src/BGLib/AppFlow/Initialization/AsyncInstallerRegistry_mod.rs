#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncInstallerRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub monoInstallers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::Zenject::MonoInstaller>,
    >,
    pub scriptableObjectInstallers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::ScriptableObjectInstaller,
        >,
    >,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::AppFlow::Initialization::AsyncInstallerRegistry =>
    "BGLib.AppFlow.Initialization"."AsyncInstallerRegistry"
);
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl std::ops::Deref for crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl std::ops::DerefMut
for crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstallerRegistry")]
impl crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry {
    pub fn AddMonoInstaller(
        &mut self,
        newMonoInstaller: quest_hook::libil2cpp::Gc<crate::Zenject::MonoInstaller>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMonoInstaller", (newMonoInstaller))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddScriptableObjectInstaller(
        &mut self,
        newScriptableObjectInstaller: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScriptableObjectInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddScriptableObjectInstaller", (newScriptableObjectInstaller))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
