#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
#[repr(C)]
#[derive(Debug)]
pub struct WrappedGeneratorStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::FilterStream,
    pub _cordl_gen: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."WrappedGeneratorStream"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::FilterStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_gen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator,
        >,
        str: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_gen, str))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_gen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator,
        >,
        str: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_gen, str))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
