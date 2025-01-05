#[cfg(feature = "ConstructorStringExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstructorStringExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "ConstructorStringExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ConstructorStringExtensions =>
    ""."ConstructorStringExtensions"
);
#[cfg(feature = "ConstructorStringExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ConstructorStringExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConstructorStringExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConstructorStringExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConstructorStringExtensions")]
impl crate::GlobalNamespace::ConstructorStringExtensions {
    pub fn ToConstructorString_Gc3<T>(
        list: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToConstructorString", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToConstructorString_Pose2(
        pose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToConstructorString", (pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToConstructorString_Quaternion1(
        quaternion: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToConstructorString", (quaternion))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToConstructorString_Vector3_0(
        vector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToConstructorString", (vector))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ConstructorStringExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConstructorStringExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
