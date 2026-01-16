#[cfg(feature = "cordl_class_BeatSaber+Destinations+DestinationBeatmapCharacteristicExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct DestinationBeatmapCharacteristicExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatSaber+Destinations+DestinationBeatmapCharacteristicExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Destinations::DestinationBeatmapCharacteristicExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Destinations";
    const CLASS_NAME: &'static str = "DestinationBeatmapCharacteristicExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatSaber+Destinations+DestinationBeatmapCharacteristicExtensions")]
impl std::ops::Deref
    for crate::BeatSaber::Destinations::DestinationBeatmapCharacteristicExtensions
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Destinations+DestinationBeatmapCharacteristicExtensions")]
impl std::ops::DerefMut
    for crate::BeatSaber::Destinations::DestinationBeatmapCharacteristicExtensions
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Destinations+DestinationBeatmapCharacteristicExtensions")]
impl crate::BeatSaber::Destinations::DestinationBeatmapCharacteristicExtensions {
    pub fn FromSerializedName(
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Destinations::BeatmapCharacteristic> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::BeatSaber::Destinations::BeatmapCharacteristic,
                        1usize,
                    >("FromSerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromSerializedName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Destinations::BeatmapCharacteristic =
            unsafe { cordl_method_info.invoke_unchecked((), (serializedName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToSerializedName(
        characteristic: crate::BeatSaber::Destinations::BeatmapCharacteristic,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BeatSaber::Destinations::BeatmapCharacteristic),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ToSerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToSerializedName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (characteristic))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Destinations+DestinationBeatmapCharacteristicExtensions")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::Destinations::DestinationBeatmapCharacteristicExtensions
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
