#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleSpawnInfoNetSerializable {
    __cordl_parent: crate::GlobalNamespace::PoolableSerializable,
    pub _cordl_time: f32,
    pub startBeat: f32,
    pub endBeat: f32,
    pub lineIndex: i32,
    pub lineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub duration: f32,
    pub width: i32,
    pub height: i32,
    pub moveOffset: crate::GlobalNamespace::Vector3Serializable,
    pub obstacleWidth: f32,
    pub obstacleHeight: f32,
    pub rotation: f32,
}
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObstacleSpawnInfoNetSerializable";
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
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable {
    type Target = crate::GlobalNamespace::PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
impl crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Deserialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Deserialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        _cordl_time: f32,
        startBeat: f32,
        endBeat: f32,
        lineIndex: i32,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
        duration: f32,
        width: i32,
        height: i32,
        moveOffset: crate::UnityEngine::Vector3,
        obstacleWidth: f32,
        obstacleHeight: f32,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    f32,
                    f32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    f32,
                    i32,
                    i32,
                    crate::UnityEngine::Vector3,
                    f32,
                    f32,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
                >,
                12usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 12usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        _cordl_time,
                        startBeat,
                        endBeat,
                        lineIndex,
                        lineLayer,
                        duration,
                        width,
                        height,
                        moveOffset,
                        obstacleWidth,
                        obstacleHeight,
                        rotation,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Obtain() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
                >,
                0usize,
            >("Obtain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Obtain", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Serialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Serialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObstacleSpawnInfoNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
