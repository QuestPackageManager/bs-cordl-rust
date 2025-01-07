#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertPairStoreSelector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub certPair: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::X509CertificatePair,
    >,
    pub forwardSelector: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
    >,
    pub reverseSelector: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
    >,
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.X509.Store";
    const CLASS_NAME: &'static str = "X509CertPairStoreSelector";
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
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
impl std::ops::Deref
for crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
impl crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CloneSelector(
        s: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CloneSelector", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Match(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Match", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509CertPairStoreSelector1(
        o: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509CertPairStoreSelector1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509CertificatePair>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509CertificatePair,
        > = __cordl_object.invoke("get_CertPair", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ForwardSelector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        > = __cordl_object.invoke("get_ForwardSelector", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReverseSelector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        > = __cordl_object.invoke("get_ReverseSelector", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CertPair(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509CertificatePair,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CertPair", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ForwardSelector(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ForwardSelector", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ReverseSelector(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::X509CertStoreSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReverseSelector", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
impl AsRef<crate::Org::BouncyCastle::X509::Store::IX509Selector>
for crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::X509::Store::IX509Selector {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
impl AsMut<crate::Org::BouncyCastle::X509::Store::IX509Selector>
for crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::X509::Store::IX509Selector {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
impl AsRef<crate::System::ICloneable>
for crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Store+X509CertPairStoreSelector")]
impl AsMut<crate::System::ICloneable>
for crate::Org::BouncyCastle::X509::Store::X509CertPairStoreSelector {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
