#[cfg(feature = "LufsMetering+SplitAudioJob")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    pub fn Create(
        interleavedData: crate::Unity::Collections::NativeArray_1<f32>,
        channelData: crate::Unity::Collections::NativeArray_1<f32>,
        numChannels: i32,
        channel: i32,
    ) -> quest_hook::libil2cpp::Result<crate::LufsMetering::SplitAudioJob> {
        let __cordl_ret: crate::LufsMetering::SplitAudioJob = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (interleavedData, channelData, numChannels, channel))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (i),
        )?;
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
