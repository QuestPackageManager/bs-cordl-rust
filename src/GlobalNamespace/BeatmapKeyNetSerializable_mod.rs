#[cfg(feature = "BeatmapKeyNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapKeyNetSerializable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _levelID_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _beatmapCharacteristicSerializedName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _difficulty_k__BackingField: crate::GlobalNamespace::BeatmapDifficulty,
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapKeyNetSerializable => ""
    ."BeatmapKeyNetSerializable"
);
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapKeyNetSerializable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapKeyNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl crate::GlobalNamespace::BeatmapKeyNetSerializable {
    pub fn Equals_BeatmapKeyNetSerializable0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_Utils_INetSerializable_Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LiteNetLib.Utils.INetSerializable.Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_Utils_INetSerializable_Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LiteNetLib.Utils.INetSerializable.Serialize", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_BeatmapDifficulty1(
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapCharacteristicSerializedName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (levelID, beatmapCharacteristicSerializedName, difficulty),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_BeatmapDifficulty1(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapCharacteristicSerializedName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (levelID, beatmapCharacteristicSerializedName, difficulty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapCharacteristicSerializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_beatmapCharacteristicSerializedName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = __cordl_object
            .invoke("get_difficulty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapCharacteristicSerializedName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapCharacteristicSerializedName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_difficulty(
        &mut self,
        value: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_difficulty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_levelID(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelID", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapKeyNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::BeatmapKeyNetSerializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::BeatmapKeyNetSerializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl AsRef<
    crate::System::IEquatable_1<*mut crate::GlobalNamespace::BeatmapKeyNetSerializable>,
> for crate::GlobalNamespace::BeatmapKeyNetSerializable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl AsMut<
    crate::System::IEquatable_1<*mut crate::GlobalNamespace::BeatmapKeyNetSerializable>,
> for crate::GlobalNamespace::BeatmapKeyNetSerializable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
