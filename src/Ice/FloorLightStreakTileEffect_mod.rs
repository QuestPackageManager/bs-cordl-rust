#[cfg(feature = "Ice+FloorLightStreakTileEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FloorLightStreakTileEffect {
    __cordl_parent: crate::GlobalNamespace::AbstractPoolContainer,
    pub _stayOnTileDuration: f32,
    pub _floorLightTilesGrid: *mut crate::Ice::FloorLightTilesGrid,
    pub _audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _elementsPool: *mut crate::GlobalNamespace::SimpleMemoryPool_1<
        *mut crate::Ice::FloorLightStreakTileEffect_Element,
    >,
}
#[cfg(feature = "Ice+FloorLightStreakTileEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Ice::FloorLightStreakTileEffect => "Ice"
    ."FloorLightStreakTileEffect"
);
#[cfg(feature = "Ice+FloorLightStreakTileEffect")]
impl std::ops::Deref for crate::Ice::FloorLightStreakTileEffect {
    type Target = crate::GlobalNamespace::AbstractPoolContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightStreakTileEffect")]
impl std::ops::DerefMut for crate::Ice::FloorLightStreakTileEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightStreakTileEffect")]
impl crate::Ice::FloorLightStreakTileEffect {
    pub const kFadeInDuration: f32 = 0.1f32;
    pub const kFadeOutDuration: f32 = 0.4f32;
    #[cfg(feature = "Ice+FloorLightStreakTileEffect+Element")]
    pub type Element = crate::Ice::FloorLightStreakTileEffect_Element;
    pub fn CreateNewElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Ice::FloorLightStreakTileEffect_Element,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Ice::FloorLightStreakTileEffect_Element = __cordl_object
            .invoke("CreateNewElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn DespawnAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleElementDidMoveToNextTile(
        &mut self,
        element: *mut crate::Ice::FloorLightStreakTileEffect_Element,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleElementDidMoveToNextTile", (element))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SpawnEffect(
        &mut self,
        x: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnEffect", (x, color))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "Ice+FloorLightStreakTileEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::Ice::FloorLightStreakTileEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Ice+FloorLightStreakTileEffect+Element")]
#[repr(C)]
#[derive(Debug)]
pub struct FloorLightStreakTileEffect_Element {
    __cordl_parent: crate::System::Object,
    pub didMoveToNextTile: *mut crate::System::Action_1<
        *mut crate::Ice::FloorLightStreakTileEffect_Element,
    >,
    pub _currentTileY: i32,
    pub _lineIndex: i32,
    pub _nextTileRemainingTime: f32,
    pub _stayOnTileDuration: f32,
    pub _color: crate::UnityEngine::Color,
}
#[cfg(feature = "Ice+FloorLightStreakTileEffect+Element")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Ice::FloorLightStreakTileEffect_Element => "Ice"
    ."FloorLightStreakTileEffect/Element"
);
#[cfg(feature = "Ice+FloorLightStreakTileEffect+Element")]
impl std::ops::Deref for crate::Ice::FloorLightStreakTileEffect_Element {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightStreakTileEffect+Element")]
impl std::ops::DerefMut for crate::Ice::FloorLightStreakTileEffect_Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightStreakTileEffect+Element")]
impl crate::Ice::FloorLightStreakTileEffect_Element {
    pub fn ManualUpdate(
        &mut self,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Setup(
        &mut self,
        color: crate::UnityEngine::Color,
        lineIndex: i32,
        stayOnTileDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (color, lineIndex, stayOnTileDuration))?;
        Ok(__cordl_ret)
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
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentTileY(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_currentTileY", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineIndex", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Ice+FloorLightStreakTileEffect+Element")]
impl quest_hook::libil2cpp::ObjectType
for crate::Ice::FloorLightStreakTileEffect_Element {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
