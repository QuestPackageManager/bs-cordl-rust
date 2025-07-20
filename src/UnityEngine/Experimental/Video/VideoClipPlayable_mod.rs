#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VideoClipPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Video";
    const CLASS_NAME: &'static str = "VideoClipPlayable";
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
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
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
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
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
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
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
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
impl crate::UnityEngine::Experimental::Video::VideoClipPlayable {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Experimental::Video::VideoClipPlayable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Experimental::Video::VideoClipPlayable),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Playables::PlayableHandle,
                        0usize,
                    >("GetHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHandle", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::Experimental::Video::VideoClipPlayable,
    >,
> for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Experimental::Video::VideoClipPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::Experimental::Video::VideoClipPlayable,
    >,
> for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Experimental::Video::VideoClipPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
impl AsRef<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Experimental+Video+VideoClipPlayable")]
impl AsMut<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Experimental::Video::VideoClipPlayable {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
