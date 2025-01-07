#[cfg(feature = "MultiplayerBadgeAwardData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeAwardData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _awardedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _weight: f32,
    pub _title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _subtitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _icon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _badgeData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerBadgeDataSO,
    >,
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerBadgeAwardData";
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
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl crate::GlobalNamespace::MultiplayerBadgeAwardData {
    pub fn CompareTo(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        awardedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        weight: f32,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subtitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        badgeData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerBadgeDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (awardedPlayer, weight, title, subtitle, badgeData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        awardedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        weight: f32,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subtitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        badgeData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerBadgeDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (awardedPlayer, weight, title, subtitle, badgeData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_awardedPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        > = __cordl_object.invoke("get_awardedPlayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_icon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = __cordl_object
            .invoke("get_icon", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subtitle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_subtitle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_title(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_title", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_titleLocalizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_titleLocalizationKey", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl AsRef<crate::System::IComparable>
for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerBadgeAwardData")]
impl AsMut<crate::System::IComparable>
for crate::GlobalNamespace::MultiplayerBadgeAwardData {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
