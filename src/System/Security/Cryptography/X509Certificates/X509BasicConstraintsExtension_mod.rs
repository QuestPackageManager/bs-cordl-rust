#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509BasicConstraintsExtension"
)]
#[repr(C)]
#[derive(Debug)]
pub struct X509BasicConstraintsExtension {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509Extension,
    pub _certificateAuthority: bool,
    pub _hasPathLengthConstraint: bool,
    pub _pathLengthConstraint: i32,
    pub _status: crate::System::Security::Cryptography::AsnDecodeStatus,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509BasicConstraintsExtension"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509BasicConstraintsExtension =>
    "System.Security.Cryptography.X509Certificates"."X509BasicConstraintsExtension"
);
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509BasicConstraintsExtension"
)]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509BasicConstraintsExtension {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509BasicConstraintsExtension"
)]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509BasicConstraintsExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509BasicConstraintsExtension"
)]
impl crate::System::Security::Cryptography::X509Certificates::X509BasicConstraintsExtension {
    pub const friendlyName: &'static str = "Basic Constraints";
    pub const oid: &'static str = "2.5.29.19";
    pub fn CopyFrom(
        &mut self,
        asnEncodedData: *mut crate::System::Security::Cryptography::AsnEncodedData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFrom", (asnEncodedData))?;
        Ok(__cordl_ret)
    }
    pub fn Decode(
        &mut self,
        extension: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::AsnDecodeStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::AsnDecodeStatus = __cordl_object
            .invoke("Decode", (extension))?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Encode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_AsnEncodedData__cordl_bool1(
        encodedBasicConstraints: *mut crate::System::Security::Cryptography::AsnEncodedData,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encodedBasicConstraints, critical))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool__cordl_bool_i32__cordl_bool2(
        certificateAuthority: bool,
        hasPathLengthConstraint: bool,
        pathLengthConstraint: i32,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificateAuthority,
                    hasPathLengthConstraint,
                    pathLengthConstraint,
                    critical,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
        multiLine: bool,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", (multiLine))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AsnEncodedData__cordl_bool1(
        &mut self,
        encodedBasicConstraints: *mut crate::System::Security::Cryptography::AsnEncodedData,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encodedBasicConstraints, critical))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool__cordl_bool_i32__cordl_bool2(
        &mut self,
        certificateAuthority: bool,
        hasPathLengthConstraint: bool,
        pathLengthConstraint: i32,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificateAuthority,
                    hasPathLengthConstraint,
                    pathLengthConstraint,
                    critical,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateAuthority(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CertificateAuthority", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasPathLengthConstraint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_HasPathLengthConstraint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PathLengthConstraint(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PathLengthConstraint", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509BasicConstraintsExtension"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509BasicConstraintsExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
