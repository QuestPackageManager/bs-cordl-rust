#[cfg(feature = "ColorTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorTypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ColorTypeExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ColorTypeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ColorTypeExtensions";
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
#[cfg(feature = "ColorTypeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ColorTypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorTypeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorTypeExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorTypeExtensions")]
impl crate::GlobalNamespace::ColorTypeExtensions {
    pub fn Opposite(
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::ColorType),
                        crate::GlobalNamespace::ColorType,
                        1usize,
                    >("Opposite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Opposite", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::ColorType = unsafe {
            cordl_method_info.invoke_unchecked((), (colorType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
