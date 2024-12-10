#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target")]
#[repr(C)]
#[derive(Debug)]
pub struct Target {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub targetName: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub targetGroup: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::Target =>
    "Org.BouncyCastle.Asn1.X509"."Target"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::Target {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::Target {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target")]
impl crate::Org::BouncyCastle::Asn1::X509::Target {
    #[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target+Choice")]
    pub type Choice = crate::Org::BouncyCastle::Asn1::X509::Target_Choice;
    pub fn New_Asn1TaggedObject0(
        tagObj: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagObj))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Target_Choice_GeneralName1(
        _cordl_type: crate::Org::BouncyCastle::Asn1::X509::Target_Choice,
        name: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, name))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1TaggedObject0(
        &mut self,
        tagObj: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagObj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Target_Choice_GeneralName1(
        &mut self,
        _cordl_type: crate::Org::BouncyCastle::Asn1::X509::Target_Choice,
        name: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::GeneralName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        > = __cordl_object.invoke("get_TargetGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::GeneralName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        > = __cordl_object.invoke("get_TargetName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::X509::Target {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Choice>
for crate::Org::BouncyCastle::Asn1::X509::Target {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Choice {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Choice>
for crate::Org::BouncyCastle::Asn1::X509::Target {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Choice {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target+Choice")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target_Choice {
    Group = 1i32,
    Name = 0i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Target+Choice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::Target_Choice =>
    "Org.BouncyCastle.Asn1.X509"."Target/Choice"
);
