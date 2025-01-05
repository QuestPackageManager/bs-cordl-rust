#[cfg(feature = "StringSignal")]
#[repr(C)]
#[derive(Debug)]
pub struct StringSignal {
    __cordl_parent: crate::GlobalNamespace::GenericSignal_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
}
#[cfg(feature = "StringSignal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StringSignal => ""
    ."StringSignal"
);
#[cfg(feature = "StringSignal")]
impl std::ops::Deref for crate::GlobalNamespace::StringSignal {
    type Target = crate::GlobalNamespace::GenericSignal_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StringSignal")]
impl std::ops::DerefMut for crate::GlobalNamespace::StringSignal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StringSignal")]
impl crate::GlobalNamespace::StringSignal {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StringSignal")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::StringSignal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
