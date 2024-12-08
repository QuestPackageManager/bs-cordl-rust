#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+EnumerableProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumerableProxy {
    __cordl_parent: crate::System::Object,
    pub inner: *mut crate::System::Collections::IEnumerable,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+EnumerableProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Utilities::Collections::EnumerableProxy =>
    "Org.BouncyCastle.Utilities.Collections"."EnumerableProxy"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+EnumerableProxy")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Utilities::Collections::EnumerableProxy {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+EnumerableProxy")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Utilities::Collections::EnumerableProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+EnumerableProxy")]
impl crate::Org::BouncyCastle::Utilities::Collections::EnumerableProxy {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        inner: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inner))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        inner: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (inner))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Collections+EnumerableProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Collections::EnumerableProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}