#[cfg(feature = "BeatSaber+Init+GameVersionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct GameVersionProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _versionTask: *mut crate::System::Threading::Tasks::Task_1<
        *mut crate::BeatSaber::Init::GameVersion,
    >,
    pub _platformInit: *mut crate::BeatSaber::Init::IPlatformInit,
}
#[cfg(feature = "BeatSaber+Init+GameVersionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::GameVersionProvider =>
    "BeatSaber.Init"."GameVersionProvider"
);
#[cfg(feature = "BeatSaber+Init+GameVersionProvider")]
impl std::ops::Deref for crate::BeatSaber::Init::GameVersionProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersionProvider")]
impl std::ops::DerefMut for crate::BeatSaber::Init::GameVersionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersionProvider")]
impl crate::BeatSaber::Init::GameVersionProvider {
    #[cfg(feature = "BeatSaber+Init+GameVersionProvider+_GetVersionInternalAsync_d__9")]
    pub type _GetVersionInternalAsync_d__9 = crate::BeatSaber::Init::GameVersionProvider__GetVersionInternalAsync_d__9;
    #[cfg(feature = "BeatSaber+Init+GameVersionProvider+_Initialize_d__5")]
    pub type _Initialize_d__5 = crate::BeatSaber::Init::GameVersionProvider__Initialize_d__5;
    pub fn GetVersionAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::BeatSaber::Init::GameVersion,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::BeatSaber::Init::GameVersion,
            >,
        > = __cordl_object.invoke("GetVersionAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVersionInternalAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::BeatSaber::Init::GameVersion,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::BeatSaber::Init::GameVersion,
            >,
        > = __cordl_object.invoke("GetVersionInternalAsync", ())?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "BeatSaber+Init+GameVersionProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Init::GameVersionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersionProvider")]
impl AsRef<crate::Zenject::IInitializable>
for crate::BeatSaber::Init::GameVersionProvider {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersionProvider")]
impl AsMut<crate::Zenject::IInitializable>
for crate::BeatSaber::Init::GameVersionProvider {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
