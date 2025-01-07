#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerHeightToJumpOffsetYProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _playerHeightDetector: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerHeightDetector,
    >,
    pub _jumpOffsetY: f32,
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerHeightToJumpOffsetYProvider";
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
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
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
    pub fn HandlePlayerHeightDidChange(
        &mut self,
        playerHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerHeightDidChange", (playerHeight))?;
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
    pub fn JumpOffsetYForPlayerHeight(
        playerHeight: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("JumpOffsetYForPlayerHeight", (playerHeight))?;
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
    pub fn get_jumpOffsetY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpOffsetY", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl AsRef<crate::GlobalNamespace::IJumpOffsetYProvider>
for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::IJumpOffsetYProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl AsMut<crate::GlobalNamespace::IJumpOffsetYProvider>
for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IJumpOffsetYProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl AsRef<crate::Zenject::IInitializable>
for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerHeightToJumpOffsetYProvider")]
impl AsMut<crate::Zenject::IInitializable>
for crate::GlobalNamespace::PlayerHeightToJumpOffsetYProvider {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
