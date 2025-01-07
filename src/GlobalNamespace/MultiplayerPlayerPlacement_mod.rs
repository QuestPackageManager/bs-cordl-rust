#[cfg(feature = "MultiplayerPlayerPlacement")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlayerPlacement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MultiplayerPlayerPlacement")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerPlayerPlacement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerPlayerPlacement";
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
#[cfg(feature = "MultiplayerPlayerPlacement")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerPlayerPlacement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayerPlacement")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerPlayerPlacement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayerPlacement")]
impl crate::GlobalNamespace::MultiplayerPlayerPlacement {
    pub fn GetAngleBetweenPlayersWithEvenAdjustment(
        numberOfPlayers: i32,
        layout: crate::GlobalNamespace::MultiplayerPlayerLayout,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetAngleBetweenPlayersWithEvenAdjustment",
                (numberOfPlayers, layout),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalPlayerIndex(
        otherPlayers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
        localPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalPlayerIndex", (otherPlayers, localPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOuterCirclePositionAngleForPlayer(
        playerIndex: i32,
        localPlayerIndex: i32,
        angleBetweenPlayers: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetOuterCirclePositionAngleForPlayer",
                (playerIndex, localPlayerIndex, angleBetweenPlayers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOuterCircleRadius(
        angleBetweenPlayers: f32,
        innerCircleRadius: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOuterCircleRadius", (angleBetweenPlayers, innerCircleRadius))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerWorldPosition(
        outerCircleRadius: f32,
        outerCirclePositionAngle: f32,
        layout: crate::GlobalNamespace::MultiplayerPlayerLayout,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetPlayerWorldPosition",
                (outerCircleRadius, outerCirclePositionAngle, layout),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SortPlayers(
        players: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortPlayers", (players))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerPlayerPlacement")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerPlayerPlacement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
