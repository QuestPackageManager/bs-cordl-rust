#[cfg(feature = "System+IO+StreamHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct StreamHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+StreamHelpers")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IO::StreamHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.IO";
    const CLASS_NAME: &'static str = "StreamHelpers";
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
#[cfg(feature = "System+IO+StreamHelpers")]
impl std::ops::Deref for crate::System::IO::StreamHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+StreamHelpers")]
impl std::ops::DerefMut for crate::System::IO::StreamHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+StreamHelpers")]
impl crate::System::IO::StreamHelpers {
    pub fn ValidateCopyToArgs(
        source: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        destination: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ValidateCopyToArgs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateCopyToArgs", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, destination, bufferSize))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+StreamHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::StreamHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
