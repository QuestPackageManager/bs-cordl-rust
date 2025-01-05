#[cfg(feature = "Unity+ThrowStub")]
#[repr(C)]
#[derive(Debug)]
pub struct ThrowStub {
    __cordl_parent: crate::System::ObjectDisposedException,
}
#[cfg(feature = "Unity+ThrowStub")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::ThrowStub => "Unity"."ThrowStub"
);
#[cfg(feature = "Unity+ThrowStub")]
impl std::ops::Deref for crate::Unity::ThrowStub {
    type Target = crate::System::ObjectDisposedException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+ThrowStub")]
impl std::ops::DerefMut for crate::Unity::ThrowStub {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+ThrowStub")]
impl crate::Unity::ThrowStub {
    pub fn ThrowNotSupportedException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowNotSupportedException", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+ThrowStub")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::ThrowStub {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
