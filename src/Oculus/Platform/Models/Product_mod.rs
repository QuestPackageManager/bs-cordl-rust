#[cfg(feature = "cordl_class_Oculus+Platform+Models+Product")]
#[repr(C)]
#[derive(Debug)]
pub struct Product {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub BillingPlansOptional:
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::BillingPlanList>,
    pub BillingPlans: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::BillingPlanList>,
    pub ContentRatingOptional:
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ContentRating>,
    pub ContentRating: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ContentRating>,
    pub CoverUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub FormattedPrice: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub IconUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Price: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Price>,
    pub ShortDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Type: crate::Oculus::Platform::ProductType,
}
#[cfg(feature = "cordl_class_Oculus+Platform+Models+Product")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Models::Product {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform.Models";
    const CLASS_NAME: &'static str = "Product";
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
#[cfg(feature = "Oculus+Platform+Models+Product")]
impl std::ops::Deref for crate::Oculus::Platform::Models::Product {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+Product")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::Product {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+Product")]
impl crate::Oculus::Platform::Models::Product {
    pub fn New(
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        ".ctor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (o))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+Models+Product")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Models::Product {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
