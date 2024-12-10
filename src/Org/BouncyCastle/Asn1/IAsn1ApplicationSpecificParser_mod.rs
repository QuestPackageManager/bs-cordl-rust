#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1ApplicationSpecificParser")]
#[repr(C)]
#[derive(Debug)]
pub struct IAsn1ApplicationSpecificParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1ApplicationSpecificParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser => "Org.BouncyCastle.Asn1"
    ."IAsn1ApplicationSpecificParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1ApplicationSpecificParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1ApplicationSpecificParser")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1ApplicationSpecificParser")]
impl crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser {
    pub fn ReadObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
        > = __cordl_object.invoke("ReadObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1ApplicationSpecificParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1ApplicationSpecificParser")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+IAsn1ApplicationSpecificParser")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}
