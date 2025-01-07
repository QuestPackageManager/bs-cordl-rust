#[cfg(feature = "Org+BouncyCastle+Crmf+RegTokenControl")]
#[repr(C)]
#[derive(Debug)]
pub struct RegTokenControl {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub token: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerUtf8String>,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+RegTokenControl")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crmf::RegTokenControl {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crmf";
    const CLASS_NAME: &'static str = "RegTokenControl";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+RegTokenControl")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::RegTokenControl {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+RegTokenControl")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::RegTokenControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+RegTokenControl")]
impl crate::Org::BouncyCastle::Crmf::RegTokenControl {
    pub fn New_DerUtf8String0(
        token: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerUtf8String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_DerUtf8String0(
        &mut self,
        token: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerUtf8String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Encodable,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+RegTokenControl")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::RegTokenControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+RegTokenControl")]
impl AsRef<crate::Org::BouncyCastle::Crmf::IControl>
for crate::Org::BouncyCastle::Crmf::RegTokenControl {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crmf::IControl {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+RegTokenControl")]
impl AsMut<crate::Org::BouncyCastle::Crmf::IControl>
for crate::Org::BouncyCastle::Crmf::RegTokenControl {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crmf::IControl {
        unsafe { std::mem::transmute(self) }
    }
}
