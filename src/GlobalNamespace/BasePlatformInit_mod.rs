#[cfg(feature = "BasePlatformInit")]
#[repr(C)]
#[derive(Debug)]
pub struct BasePlatformInit {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _IsInitialized_k__BackingField: bool,
    pub _initializationTask: *mut crate::System::Threading::Tasks::Task_1<bool>,
}
#[cfg(feature = "BasePlatformInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BasePlatformInit => ""
    ."BasePlatformInit"
);
#[cfg(feature = "BasePlatformInit")]
impl std::ops::Deref for crate::GlobalNamespace::BasePlatformInit {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BasePlatformInit")]
impl std::ops::DerefMut for crate::GlobalNamespace::BasePlatformInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BasePlatformInit")]
impl crate::GlobalNamespace::BasePlatformInit {
    #[cfg(feature = "BasePlatformInit+_InitializeAsync_d__7")]
    pub type _InitializeAsync_d__7 = crate::GlobalNamespace::BasePlatformInit__InitializeAsync_d__7;
    #[cfg(feature = "BasePlatformInit+_Initialize_d__5")]
    pub type _Initialize_d__5 = crate::GlobalNamespace::BasePlatformInit__Initialize_d__5;
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("InitializeAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeInternalAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("InitializeInternalAsync", ())?;
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
    pub fn get_IsInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsInitialized(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsInitialized", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BasePlatformInit")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BasePlatformInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BasePlatformInit")]
impl AsRef<crate::GlobalNamespace::IPlatformInit>
for crate::GlobalNamespace::BasePlatformInit {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPlatformInit {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BasePlatformInit")]
impl AsMut<crate::GlobalNamespace::IPlatformInit>
for crate::GlobalNamespace::BasePlatformInit {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPlatformInit {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BasePlatformInit")]
impl AsRef<crate::Zenject::IInitializable> for crate::GlobalNamespace::BasePlatformInit {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BasePlatformInit")]
impl AsMut<crate::Zenject::IInitializable> for crate::GlobalNamespace::BasePlatformInit {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
