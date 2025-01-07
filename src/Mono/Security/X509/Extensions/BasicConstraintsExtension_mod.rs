#[cfg(feature = "Mono+Security+X509+Extensions+BasicConstraintsExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicConstraintsExtension {
    __cordl_parent: crate::Mono::Security::X509::X509Extension,
    pub cA: bool,
    pub pathLenConstraint: i32,
}
#[cfg(feature = "Mono+Security+X509+Extensions+BasicConstraintsExtension")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Security::X509::Extensions::BasicConstraintsExtension {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Security.X509.Extensions";
    const CLASS_NAME: &'static str = "BasicConstraintsExtension";
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
#[cfg(feature = "Mono+Security+X509+Extensions+BasicConstraintsExtension")]
impl std::ops::Deref
for crate::Mono::Security::X509::Extensions::BasicConstraintsExtension {
    type Target = crate::Mono::Security::X509::X509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+Extensions+BasicConstraintsExtension")]
impl std::ops::DerefMut
for crate::Mono::Security::X509::Extensions::BasicConstraintsExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+Extensions+BasicConstraintsExtension")]
impl crate::Mono::Security::X509::Extensions::BasicConstraintsExtension {
    pub fn Decode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Decode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        extension: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Extension>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (extension))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        extension: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Extension>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (extension))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateAuthority(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CertificateAuthority", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+X509+Extensions+BasicConstraintsExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::X509::Extensions::BasicConstraintsExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
