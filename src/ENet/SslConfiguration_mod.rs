#[cfg(feature = "ENet+SslConfiguration")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SslConfiguration {
    pub nativeSslConfiguration: crate::ENet::ENetSslConfiguration,
}
#[cfg(feature = "ENet+SslConfiguration")]
unsafe impl quest_hook::libil2cpp::Type for crate::ENet::SslConfiguration {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "ENet";
    const CLASS_NAME: &'static str = "SslConfiguration";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "ENet+SslConfiguration")]
unsafe impl quest_hook::libil2cpp::Argument for crate::ENet::SslConfiguration {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ENet+SslConfiguration")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::ENet::SslConfiguration {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "ENet+SslConfiguration")]
unsafe impl quest_hook::libil2cpp::Returned for crate::ENet::SslConfiguration {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "ENet+SslConfiguration")]
unsafe impl quest_hook::libil2cpp::Return for crate::ENet::SslConfiguration {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "ENet+SslConfiguration")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::SslConfiguration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+SslConfiguration")]
impl crate::ENet::SslConfiguration {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sslConfiguration: crate::ENet::ENetSslConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (sslConfiguration),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Certificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Certificate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificatePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_CertificatePath",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HostName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_HostName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mode(&mut self) -> quest_hook::libil2cpp::Result<crate::ENet::SslMode> {
        let __cordl_ret: crate::ENet::SslMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Mode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NativeSslConfiguration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::ENetSslConfiguration> {
        let __cordl_ret: crate::ENet::ENetSslConfiguration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NativeSslConfiguration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_PrivateKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrivateKeyPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_PrivateKeyPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RootCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RootCertificate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RootCertificatePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RootCertificatePath",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValidateCertificate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ValidateCertificate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Certificate(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Certificate",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CertificatePath(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_CertificatePath",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HostName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_HostName",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Mode(
        &mut self,
        value: crate::ENet::SslMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Mode",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NativeSslConfiguration(
        &mut self,
        value: crate::ENet::ENetSslConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_NativeSslConfiguration",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PrivateKey(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_PrivateKey",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PrivateKeyPath(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_PrivateKeyPath",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RootCertificate(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_RootCertificate",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RootCertificatePath(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_RootCertificatePath",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ValidateCertificate(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ValidateCertificate",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ENet+SslConfiguration")]
impl AsRef<crate::System::IDisposable> for crate::ENet::SslConfiguration {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "ENet+SslConfiguration")]
impl AsMut<crate::System::IDisposable> for crate::ENet::SslConfiguration {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
