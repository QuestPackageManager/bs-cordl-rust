#[cfg(feature = "BeatmapKeyNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapKeyNetSerializable {
    __cordl_parent: crate::System::Object,
    pub _levelID_k__BackingField: *mut crate::System::String,
    pub _beatmapCharacteristicSerializedName_k__BackingField: *mut crate::System::String,
    pub _difficulty_k__BackingField: BeatmapDifficulty,
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapKeyNetSerializable => ""
    ."BeatmapKeyNetSerializable"
);
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl std::ops::Deref for BeatmapKeyNetSerializable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl std::ops::DerefMut for BeatmapKeyNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl BeatmapKeyNetSerializable {
    pub fn Equals_BeatmapKeyNetSerializable0(
        &mut self,
        other: *mut BeatmapKeyNetSerializable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_Utils_INetSerializable_Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LiteNetLib.Utils.INetSerializable.Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_Utils_INetSerializable_Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LiteNetLib.Utils.INetSerializable.Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_BeatmapDifficulty1(
        levelID: *mut crate::System::String,
        beatmapCharacteristicSerializedName: *mut crate::System::String,
        difficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (levelID, beatmapCharacteristicSerializedName, difficulty),
            )?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_BeatmapDifficulty1(
        &mut self,
        levelID: *mut crate::System::String,
        beatmapCharacteristicSerializedName: *mut crate::System::String,
        difficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (levelID, beatmapCharacteristicSerializedName, difficulty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapCharacteristicSerializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_beatmapCharacteristicSerializedName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_difficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapDifficulty = __cordl_object
            .invoke("get_difficulty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_levelID", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapCharacteristicSerializedName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapCharacteristicSerializedName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_difficulty(
        &mut self,
        value: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_difficulty", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_levelID(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelID", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapKeyNetSerializable")]
impl quest_hook::libil2cpp::ObjectType for BeatmapKeyNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}