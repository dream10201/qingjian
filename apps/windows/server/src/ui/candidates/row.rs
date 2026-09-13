//! 候选窗口的一行：[`Candidate`] 的展示形态，与 macOS 端 `candidates/row.rs` 一致。

use qingjian_core::{Candidate, CandidateKind};

/// annotation 片段的深浅。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tone {
    /// 附注（emoji 对应的词、问字答案的读音）。
    Gloss,
}

/// 候选窗口的一行。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Row {
    /// 显示用序号文本，如 `1`。
    pub index: String,

    /// 候选词本体。
    pub text: String,

    /// 右侧 annotation，按顺序绘制；没有附注时为空。
    pub annotation: Vec<(String, Tone)>,

    /// 来自云联想：词前画一个小云朵。
    pub cloud: bool,
}

impl Row {
    /// `position` 是页内下标（从 0 起）。
    pub(crate) fn from_candidate(position: usize, candidate: &Candidate) -> Self {
        let annotation = candidate
            .reading
            .iter()
            .map(|reading| (reading.clone(), Tone::Gloss))
            .collect();
        Self {
            index: (position + 1).to_string(),
            text: candidate.text.clone(),
            annotation,
            cloud: candidate.kind == CandidateKind::Cloud,
        }
    }
}
