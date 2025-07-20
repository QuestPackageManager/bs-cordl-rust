#[cfg(feature = "OVRSceneRoom")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneRoom {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _Floor_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRScenePlane,
    >,
    pub _Ceiling_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRScenePlane,
    >,
    pub _Walls_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
        >,
    >,
    pub _walls: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
        >,
    >,
    pub _orderedRoomGuids: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<crate::System::Guid, i32>,
    >,
    pub _wallOrderComparer: quest_hook::libil2cpp::Gc<
        crate::System::Comparison_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
        >,
    >,
    pub _sceneAnchor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSceneAnchor>,
    pub _sceneManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRSceneManager,
    >,
    pub _uuidToQuery: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<crate::System::Guid>,
    >,
    pub _roomAnchors: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRAnchor>,
    >,
    pub _taskCount: i32,
    pub _onFetchAnchorsCompleted: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<bool>,
    >,
    pub _onAnchorLocalizationCompleted: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<bool, crate::GlobalNamespace::OVRAnchor>,
    >,
}
#[cfg(feature = "OVRSceneRoom")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRSceneRoom {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRSceneRoom";
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
#[cfg(feature = "OVRSceneRoom")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneRoom {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSceneRoom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl crate::GlobalNamespace::OVRSceneRoom {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "Awake", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUuidsToQuery(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("GetUuidsToQuery")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "GetUuidsToQuery", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IOVRSceneComponent_Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("IOVRSceneComponent.Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "IOVRSceneComponent.Initialize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRoom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LoadRoom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "LoadRoom", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
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
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnFetchAnchorsCompleted(
        &mut self,
        success: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnFetchAnchorsCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "OnFetchAnchorsCompleted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (success))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnLocalizationCompleted(
        &mut self,
        success: bool,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, crate::GlobalNamespace::OVRAnchor),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnLocalizationCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "OnLocalizationCompleted", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (success, anchor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRoomInformation(
        &mut self,
        plane: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UpdateRoomInformation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "UpdateRoomInformation", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (plane))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__24_0(
        &mut self,
        planeA: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
        planeB: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
                ),
                i32,
                2usize,
            >("<Awake>b__24_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "<Awake>b__24_0", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (planeA, planeB))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_g__TryGetUuid_24_1(
        &mut self,
        plane: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                2usize,
            >("<Awake>g__TryGetUuid|24_1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "<Awake>g__TryGetUuid|24_1", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (plane, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Ceiling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
                0usize,
            >("get_Ceiling")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "get_Ceiling", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRScenePlane,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Floor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
                0usize,
            >("get_Floor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "get_Floor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRScenePlane,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Walls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
                    >,
                >,
                0usize,
            >("get_Walls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "get_Walls", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Ceiling(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Ceiling")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "set_Ceiling", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Floor(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Floor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "set_Floor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Walls(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRScenePlane>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Walls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::OVRSceneRoom as quest_hook::libil2cpp::Type
                    > ::class(), "set_Walls", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRSceneRoom {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl AsRef<crate::GlobalNamespace::IOVRSceneComponent>
for crate::GlobalNamespace::OVRSceneRoom {
    fn as_ref(&self) -> &crate::GlobalNamespace::IOVRSceneComponent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRSceneRoom")]
impl AsMut<crate::GlobalNamespace::IOVRSceneComponent>
for crate::GlobalNamespace::OVRSceneRoom {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IOVRSceneComponent {
        unsafe { std::mem::transmute(self) }
    }
}
