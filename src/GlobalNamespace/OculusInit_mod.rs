#[cfg(feature = "OculusInit")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusInit {
    __cordl_parent: BasePlatformInit,
    pub _playerDataModel: *mut PlayerDataModel,
}
#[cfg(feature = "OculusInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusInit => ""."OculusInit"
);
#[cfg(feature = "OculusInit")]
impl std::ops::Deref for OculusInit {
    type Target = BasePlatformInit;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusInit")]
impl std::ops::DerefMut for OculusInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusInit")]
impl OculusInit {
    pub const kPlatformNotInstalledMessage: &'static str = "Install the Oculus app to support Oculus platform on PC from: https://www.meta.com/help/quest/articles/getting-started/getting-started-with-rift-s/install-oculus-pc-app/";
    #[cfg(feature = "OculusInit+_InitializeOculusAsync_d__4")]
    pub type _InitializeOculusAsync_d__4 = crate::GlobalNamespace::OculusInit__InitializeOculusAsync_d__4;
    #[cfg(feature = "OculusInit+_CheckEntitlementsAsync_d__6")]
    pub type _CheckEntitlementsAsync_d__6 = crate::GlobalNamespace::OculusInit__CheckEntitlementsAsync_d__6;
    #[cfg(feature = "OculusInit+_CheckUserAgeCategoryAsync_d__7")]
    pub type _CheckUserAgeCategoryAsync_d__7 = crate::GlobalNamespace::OculusInit__CheckUserAgeCategoryAsync_d__7;
    #[cfg(feature = "OculusInit+_InitializeInternalAsync_d__3")]
    pub type _InitializeInternalAsync_d__3 = crate::GlobalNamespace::OculusInit__InitializeInternalAsync_d__3;
    #[cfg(feature = "OculusInit+_InitializeCoreAsync_d__5")]
    pub type _InitializeCoreAsync_d__5 = crate::GlobalNamespace::OculusInit__InitializeCoreAsync_d__5;
    pub fn CheckUserAgeCategoryAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("CheckUserAgeCategoryAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeInternalAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("InitializeInternalAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeOculusAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("InitializeOculusAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OculusInit")]
impl quest_hook::libil2cpp::ObjectType for OculusInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
