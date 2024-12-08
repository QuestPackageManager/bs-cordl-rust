#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartSO_1")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarPartSO_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: PersistentScriptableObject,
    pub _id: *mut crate::System::String,
    pub _localizationKey: *mut crate::System::String,
    pub _partAsset: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartSO_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::AvatarPartSO_1 < T >
    => "BeatSaber.BeatAvatarSDK"."AvatarPartSO`1" < T >
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartSO_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::BeatSaber::BeatAvatarSDK::AvatarPartSO_1<T> {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartSO_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::BeatSaber::BeatAvatarSDK::AvatarPartSO_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartSO_1")]
impl<T: quest_hook::libil2cpp::Type> crate::BeatSaber::BeatAvatarSDK::AvatarPartSO_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<*mut crate::System::String>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_localizationKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_partAsset(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_partAsset", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartSO_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::AvatarPartSO_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}