#[cfg(feature = "System+Nullable")]
#[repr(C)]
#[derive(Debug)]
pub struct Nullable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Nullable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Nullable => "System"."Nullable"
);
#[cfg(feature = "System+Nullable")]
impl std::ops::Deref for crate::System::Nullable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Nullable")]
impl std::ops::DerefMut for crate::System::Nullable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Nullable")]
impl crate::System::Nullable {
    pub fn GetUnderlyingType(
        nullableType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnderlyingType", (nullableType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Nullable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Nullable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
