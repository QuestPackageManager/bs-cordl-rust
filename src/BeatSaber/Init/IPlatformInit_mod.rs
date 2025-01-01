#[cfg(feature = "BeatSaber+Init+IPlatformInit")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlatformInit {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Init+IPlatformInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::IPlatformInit =>
    "BeatSaber.Init"."IPlatformInit"
);
#[cfg(feature = "BeatSaber+Init+IPlatformInit")]
impl std::ops::Deref for crate::BeatSaber::Init::IPlatformInit {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+IPlatformInit")]
impl std::ops::DerefMut for crate::BeatSaber::Init::IPlatformInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+IPlatformInit")]
impl crate::BeatSaber::Init::IPlatformInit {
    pub fn GetAppVersionAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("GetAppVersionAsync", ())?;
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_IsInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitialized", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+IPlatformInit")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Init::IPlatformInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Init+IPlatformInit")]
impl AsRef<crate::Zenject::IInitializable> for crate::BeatSaber::Init::IPlatformInit {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+IPlatformInit")]
impl AsMut<crate::Zenject::IInitializable> for crate::BeatSaber::Init::IPlatformInit {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
