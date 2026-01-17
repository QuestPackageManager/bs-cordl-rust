#[cfg(feature = "cordl_class_System+Resources+ResourceFallbackManager")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ResourceFallbackManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_startingCulture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    pub m_neutralResourcesCulture:
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    pub m_useParents: bool,
}
#[cfg(feature = "cordl_class_System+Resources+ResourceFallbackManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Resources::ResourceFallbackManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Resources";
    const CLASS_NAME: &'static str = "ResourceFallbackManager";
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
#[cfg(feature = "System+Resources+ResourceFallbackManager")]
impl std::ops::Deref for crate::System::Resources::ResourceFallbackManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ResourceFallbackManager")]
impl std::ops::DerefMut for crate::System::Resources::ResourceFallbackManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+ResourceFallbackManager")]
impl crate::System::Resources::ResourceFallbackManager {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerator_1<
                            quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                        >,
                    >, 0usize>("GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetEnumerator",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        startingCulture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        neutralResourcesCulture: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        >,
        useParents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (startingCulture, neutralResourcesCulture, useParents),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        0usize,
                    >("System.Collections.IEnumerable.GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerable.GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        startingCulture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        neutralResourcesCulture: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        >,
        useParents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (startingCulture, neutralResourcesCulture, useParents))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Resources+ResourceFallbackManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Resources::ResourceFallbackManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Resources+ResourceFallbackManager")]
impl
    AsRef<
        crate::System::Collections::Generic::IEnumerable_1<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        >,
    > for crate::System::Resources::ResourceFallbackManager
{
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+ResourceFallbackManager")]
impl
    AsMut<
        crate::System::Collections::Generic::IEnumerable_1<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        >,
    > for crate::System::Resources::ResourceFallbackManager
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+ResourceFallbackManager")]
impl AsRef<crate::System::Collections::IEnumerable>
    for crate::System::Resources::ResourceFallbackManager
{
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+ResourceFallbackManager")]
impl AsMut<crate::System::Collections::IEnumerable>
    for crate::System::Resources::ResourceFallbackManager
{
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
