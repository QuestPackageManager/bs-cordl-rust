#[cfg(feature = "Unity+Properties+PropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyBag {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Properties+PropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::PropertyBag =>
    "Unity.Properties"."PropertyBag"
);
#[cfg(feature = "Unity+Properties+PropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::PropertyBag {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag")]
impl std::ops::DerefMut for crate::Unity::Properties::PropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag")]
impl crate::Unity::Properties::PropertyBag {
    pub fn Register<TContainer>(
        propertyBag: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::PropertyBag_1<TContainer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Register", (propertyBag))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Properties::PropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
