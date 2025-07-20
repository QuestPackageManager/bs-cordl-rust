#[cfg(feature = "GameServerListTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerListTableCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _serverName: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedTextMeshPro>,
    pub _difficultiesText: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedTextMeshPro>,
    pub _musicPackText: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedTextMeshPro>,
    pub _playerCount: quest_hook::libil2cpp::Gc<crate::HMUI::CurvedTextMeshPro>,
    pub _passwordProtected: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _songPackMasksModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPackMasksModel,
    >,
}
#[cfg(feature = "GameServerListTableCell")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameServerListTableCell {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameServerListTableCell";
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
#[cfg(feature = "GameServerListTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerListTableCell {
    type Target = crate::HMUI::TableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerListTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerListTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerListTableCell")]
impl crate::GlobalNamespace::GameServerListTableCell {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetData(
        &mut self,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::INetworkPlayer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (player))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameServerListTableCell")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServerListTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
