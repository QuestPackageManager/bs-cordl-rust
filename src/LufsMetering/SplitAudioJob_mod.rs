#[cfg(feature = "LufsMetering+SplitAudioJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SplitAudioJob {
    pub channelData: crate::Unity::Collections::NativeArray_1<f32>,
    pub interleavedData: crate::Unity::Collections::NativeArray_1<f32>,
    pub mumChannels: i32,
    pub channel: i32,
}
#[cfg(feature = "LufsMetering+SplitAudioJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LufsMetering::SplitAudioJob => "LufsMetering"
    ."SplitAudioJob"
);
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
    pub fn Execute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (i),
        )?;
        Ok(__cordl_ret)
    }
}
