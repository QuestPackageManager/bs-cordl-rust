#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct UITKTextJobSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub textJobDatasHandle: crate::System::Runtime::InteropServices::GCHandle,
    pub textJobDatas: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData,
            >,
        >,
    >,
    pub hasPendingTextWork: bool,
    pub m_PrepareTextJobifiedCallback:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshGenerationCallback>,
    pub m_GenerateTextJobifiedCallback:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshGenerationCallback>,
    pub m_AddDrawEntriesCallback:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshGenerationCallback>,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::UITKTextJobSystem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UITKTextJobSystem";
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
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UITKTextJobSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UITKTextJobSystem {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem")]
impl crate::UnityEngine::UIElements::UITKTextJobSystem {
    #[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
    pub type GenerateTextJobData =
        crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData;
    #[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+ManagedJobData")]
    pub type ManagedJobData = crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData;
    #[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
    pub type PrepareTextJobData =
        crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData;
    pub fn AddDrawEntries(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl__: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>("AddDrawEntries")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddDrawEntries",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mgc, _cordl__))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertMeshInfoToUIRVertex(
        meshInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TextCore::Text::MeshInfo>,
        >,
        alloc: crate::UnityEngine::UIElements::TempMeshAllocator,
        visualElement: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
        materials: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                >,
            >,
        >,
        verticesArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::Unity::Collections::NativeSlice_1<
                        crate::UnityEngine::UIElements::Vertex,
                    >,
                >,
            >,
        >,
        indicesArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::Unity::Collections::NativeSlice_1<u16>,
                >,
            >,
        >,
        renderModes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::Text::MeshInfo,
                            >,
                        >,
                        crate::UnityEngine::UIElements::TempMeshAllocator,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::Unity::Collections::NativeSlice_1<
                                        crate::UnityEngine::UIElements::Vertex,
                                    >,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::Unity::Collections::NativeSlice_1<u16>,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
                                >,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "ConvertMeshInfoToUIRVertex"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertMeshInfoToUIRVertex",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    meshInfos,
                    alloc,
                    visualElement,
                    materials,
                    verticesArray,
                    indicesArray,
                    renderModes,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateText(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        textElement: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
                    ), quest_hook::libil2cpp::Void, 2usize>("GenerateText")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateText",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mgc, textElement))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTextJobified(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl__: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "GenerateTextJobified"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateTextJobified",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mgc, _cordl__))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnGetManagedJob(
        managedJobData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData,
                    >), quest_hook::libil2cpp::Void, 1usize>("OnGetManagedJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnGetManagedJob",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (managedJobData))? };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareTextJobified(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl__: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "PrepareTextJobified"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareTextJobified",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mgc, _cordl__))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UITKTextJobSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct UITKTextJobSystem_GenerateTextJobData {
    pub managedJobDataHandle: crate::System::Runtime::InteropServices::GCHandle,
    pub alloc: crate::UnityEngine::UIElements::TempMeshAllocator,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UITKTextJobSystem/GenerateTextJobData";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
impl crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+GenerateTextJobData")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::UIElements::UITKTextJobSystem_GenerateTextJobData
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+ManagedJobData")]
#[repr(C)]
#[derive(Debug)]
pub struct UITKTextJobSystem_ManagedJobData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub visualElement: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    pub node: crate::UnityEngine::UIElements::MeshGenerationNode,
    pub materials: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    >,
    pub renderModes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        >,
    >,
    pub vertices: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        >,
    >,
    pub indices: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::Unity::Collections::NativeSlice_1<u16>>,
    >,
    pub prepareSuccess: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+ManagedJobData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UITKTextJobSystem/ManagedJobData";
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
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+ManagedJobData")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+ManagedJobData")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+ManagedJobData")]
impl crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Release")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Release",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+ManagedJobData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::UITKTextJobSystem_ManagedJobData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct UITKTextJobSystem_PrepareTextJobData {
    pub managedJobDataHandle: crate::System::Runtime::InteropServices::GCHandle,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UITKTextJobSystem/PrepareTextJobData";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
impl crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UITKTextJobSystem+PrepareTextJobData")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::UIElements::UITKTextJobSystem_PrepareTextJobData
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
