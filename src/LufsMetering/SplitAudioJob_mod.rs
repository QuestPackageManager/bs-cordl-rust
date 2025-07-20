#[cfg(feature = "LufsMetering+SplitAudioJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SplitAudioJob {
    pub channelData: crate::Unity::Collections::NativeArray_1<f32>,
    pub interleavedData: crate::Unity::Collections::NativeArray_1<f32>,
    pub mumChannels: i32,
    pub channel: i32,
}
#[cfg(feature = "LufsMetering+SplitAudioJob")]
unsafe impl quest_hook::libil2cpp::Type for crate::LufsMetering::SplitAudioJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "LufsMetering";
    const CLASS_NAME: &'static str = "SplitAudioJob";
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
#[cfg(feature = "LufsMetering+SplitAudioJob")]
unsafe impl quest_hook::libil2cpp::Argument for crate::LufsMetering::SplitAudioJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LufsMetering+SplitAudioJob")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::LufsMetering::SplitAudioJob {
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
#[cfg(feature = "LufsMetering+SplitAudioJob")]
unsafe impl quest_hook::libil2cpp::Returned for crate::LufsMetering::SplitAudioJob {
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
#[cfg(feature = "LufsMetering+SplitAudioJob")]
unsafe impl quest_hook::libil2cpp::Return for crate::LufsMetering::SplitAudioJob {
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
#[cfg(feature = "LufsMetering+SplitAudioJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LufsMetering::SplitAudioJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LufsMetering+SplitAudioJob")]
impl crate::LufsMetering::SplitAudioJob {
    pub fn Create(
        interleavedData: crate::Unity::Collections::NativeArray_1<f32>,
        channelData: crate::Unity::Collections::NativeArray_1<f32>,
        numChannels: i32,
        channel: i32,
    ) -> quest_hook::libil2cpp::Result<crate::LufsMetering::SplitAudioJob> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<f32>,
                            crate::Unity::Collections::NativeArray_1<f32>,
                            i32,
                            i32,
                        ),
                        crate::LufsMetering::SplitAudioJob,
                        4usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Create", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::LufsMetering::SplitAudioJob = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (interleavedData, channelData, numChannels, channel),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Execute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (i))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LufsMetering+SplitAudioJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor> for crate::LufsMetering::SplitAudioJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "LufsMetering+SplitAudioJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor> for crate::LufsMetering::SplitAudioJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
