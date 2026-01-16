#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
#[repr(C)]
#[derive(Debug)]
pub struct WrappedGeneratorStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::FilterStream,
    pub _cordl_gen:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator>,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "WrappedGeneratorStream";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::FilterStream;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream {
    pub fn Close(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Close")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Close",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_gen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator,
        >,
        str: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_gen, str))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Bcpg+OpenPgp+WrappedGeneratorStream")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Bcpg::OpenPgp::WrappedGeneratorStream
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
