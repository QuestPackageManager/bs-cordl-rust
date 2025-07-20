#[cfg(feature = "CoreMathUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct CoreMathUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "CoreMathUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::CoreMathUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CoreMathUtils";
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
#[cfg(feature = "CoreMathUtils")]
impl std::ops::Deref for crate::GlobalNamespace::CoreMathUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CoreMathUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::CoreMathUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CoreMathUtils")]
impl crate::GlobalNamespace::CoreMathUtils {
    pub const kHalfJumpDistanceEpsilon: f32 = 0.001f32;
    pub fn CalculateHalfJumpDurationInBeats(
        startHalfJumpDurationInBeats: f32,
        maxHalfJumpDistance: f32,
        noteJumpMovementSpeed: f32,
        oneBeatDuration: f32,
        noteJumpStartBeatOffset: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::CoreMathUtils as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32, f32),
                f32,
                5usize,
            >("CalculateHalfJumpDurationInBeats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::CoreMathUtils as quest_hook::libil2cpp::Type
                    > ::class(), "CalculateHalfJumpDurationInBeats", 5usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        startHalfJumpDurationInBeats,
                        maxHalfJumpDistance,
                        noteJumpMovementSpeed,
                        oneBeatDuration,
                        noteJumpStartBeatOffset,
                    ),
                )?
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
    pub fn __CalculateHalfJumpDurationInBeatsV1(
        startHalfJumpDurationInBeats: f32,
        maxHalfJumpDistance: f32,
        noteJumpMovementSpeed: f32,
        oneBeatDuration: f32,
        minHalfJumpDistance: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::CoreMathUtils as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32, f32),
                i32,
                5usize,
            >("__CalculateHalfJumpDurationInBeatsV1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::CoreMathUtils as quest_hook::libil2cpp::Type
                    > ::class(), "__CalculateHalfJumpDurationInBeatsV1", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        startHalfJumpDurationInBeats,
                        maxHalfJumpDistance,
                        noteJumpMovementSpeed,
                        oneBeatDuration,
                        minHalfJumpDistance,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn __CalculateHalfJumpDurationInBeatsV2(
        startHalfJumpDurationInBeats: f32,
        maxHalfJumpDistance: f32,
        noteJumpMovementSpeed: f32,
        oneBeatDuration: f32,
        noteJumpStartBeatOffset: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::CoreMathUtils as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32, f32),
                f32,
                5usize,
            >("__CalculateHalfJumpDurationInBeatsV2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::CoreMathUtils as quest_hook::libil2cpp::Type
                    > ::class(), "__CalculateHalfJumpDurationInBeatsV2", 5usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        startHalfJumpDurationInBeats,
                        maxHalfJumpDistance,
                        noteJumpMovementSpeed,
                        oneBeatDuration,
                        noteJumpStartBeatOffset,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::CoreMathUtils as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::CoreMathUtils as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CoreMathUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CoreMathUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
