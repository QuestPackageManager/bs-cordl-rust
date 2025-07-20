#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackAssetExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TrackAssetExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TrackAssetExtensions";
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
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackAssetExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TrackAssetExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
impl crate::UnityEngine::Timeline::TrackAssetExtensions {
    pub fn GetGroup(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::GroupTrack>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TrackAssetExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::GroupTrack>,
                1usize,
            >("GetGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TrackAssetExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "GetGroup", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::GroupTrack,
        > = unsafe { method.invoke_unchecked((), (asset))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGroup(
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
        group: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::GroupTrack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Timeline::TrackAssetExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::GroupTrack>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Timeline::TrackAssetExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "SetGroup", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (asset, group))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackAssetExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TrackAssetExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
