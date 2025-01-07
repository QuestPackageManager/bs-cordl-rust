#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSender")]
#[repr(C)]
#[derive(Debug)]
pub struct OptionalAvatarDataSender {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _avatarSystemCollection: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    >,
    pub _optionalAvatarDataSyncHandler: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
    >,
    pub _optionalAvatarDataProvider: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
    >,
    pub _currentAvatarDisplayContext: crate::BeatSaber::AvatarCore::AvatarDisplayContext,
    pub _selectedAvatarTypeId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSender")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::OptionalAvatarDataSender {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "OptionalAvatarDataSender";
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
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSender")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::OptionalAvatarDataSender {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSender")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::OptionalAvatarDataSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSender")]
impl crate::BeatSaber::AvatarCore::OptionalAvatarDataSender {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleOptionalAvatarDataProviderDataDidChange(
        &mut self,
        data: crate::BeatSaber::AvatarCore::OptionalAvatarData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOptionalAvatarDataProviderDataDidChange", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        avatarSystemCollection: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::AvatarSystemCollection,
        >,
        optionalAvatarDataSyncHandler: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (avatarSystemCollection, optionalAvatarDataSyncHandler),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetCurrentLocalAvatarDisplayContext(
        &mut self,
        avatarDisplayContext: crate::BeatSaber::AvatarCore::AvatarDisplayContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurrentLocalAvatarDisplayContext", (avatarDisplayContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSelectedAvatarTypeId(
        &mut self,
        selectedAvatarTypeId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectedAvatarTypeId", (selectedAvatarTypeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        avatarSystemCollection: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::AvatarSystemCollection,
        >,
        optionalAvatarDataSyncHandler: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::OptionalAvatarDataSyncHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (avatarSystemCollection, optionalAvatarDataSyncHandler))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSender")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::OptionalAvatarDataSender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSender")]
impl AsRef<crate::System::IDisposable>
for crate::BeatSaber::AvatarCore::OptionalAvatarDataSender {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataSender")]
impl AsMut<crate::System::IDisposable>
for crate::BeatSaber::AvatarCore::OptionalAvatarDataSender {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
