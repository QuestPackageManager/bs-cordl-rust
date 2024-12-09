#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultGameObjectParentInstaller {
    __cordl_parent: crate::Zenject::Installer_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::Zenject::DefaultGameObjectParentInstaller,
    >,
    pub _name: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::DefaultGameObjectParentInstaller =>
    "Zenject"."DefaultGameObjectParentInstaller"
);
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller")]
impl std::ops::Deref for crate::Zenject::DefaultGameObjectParentInstaller {
    type Target = crate::Zenject::Installer_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::Zenject::DefaultGameObjectParentInstaller,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller")]
impl std::ops::DerefMut for crate::Zenject::DefaultGameObjectParentInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller")]
impl crate::Zenject::DefaultGameObjectParentInstaller {
    #[cfg(
        feature = "Zenject+DefaultGameObjectParentInstaller+DefaultParentObjectDestroyer"
    )]
    pub type DefaultParentObjectDestroyer = crate::Zenject::DefaultGameObjectParentInstaller_DefaultParentObjectDestroyer;
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::DefaultGameObjectParentInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller+DefaultParentObjectDestroyer")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultGameObjectParentInstaller_DefaultParentObjectDestroyer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameObject: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller+DefaultParentObjectDestroyer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::DefaultGameObjectParentInstaller_DefaultParentObjectDestroyer => "Zenject"
    ."DefaultGameObjectParentInstaller/DefaultParentObjectDestroyer"
);
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller+DefaultParentObjectDestroyer")]
impl std::ops::Deref
for crate::Zenject::DefaultGameObjectParentInstaller_DefaultParentObjectDestroyer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller+DefaultParentObjectDestroyer")]
impl std::ops::DerefMut
for crate::Zenject::DefaultGameObjectParentInstaller_DefaultParentObjectDestroyer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller+DefaultParentObjectDestroyer")]
impl crate::Zenject::DefaultGameObjectParentInstaller_DefaultParentObjectDestroyer {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (gameObject))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (gameObject))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+DefaultGameObjectParentInstaller+DefaultParentObjectDestroyer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::DefaultGameObjectParentInstaller_DefaultParentObjectDestroyer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
