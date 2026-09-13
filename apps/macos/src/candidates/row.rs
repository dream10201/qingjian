//! 候选窗口的一行：序号、候选词、annotation 片段。只是 Core 输出的展示形态，不含任何排序或查词。

use qingjian_core::Candidate;

/// annotation 片段的深浅。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    /// 附注（emoji 对应的词、问字答案的读音）。
    Gloss,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// 显示用序号文本，如 `1`。
    pub index: String,

    /// 候选词。
    pub text: String,

    /// 右侧 annotation，按顺序绘制；没有附注时为空。
    pub annotation: Vec<(String, Tone)>,

    /// 来自云联想：词前画一个小云朵，与本地候选区分。
    pub cloud: bool,
}

impl Row {
    pub fn from_candidate(position: usize, candidate: &Candidate) -> Self {
        let annotation = candidate
            .reading
            .iter()
            .map(|reading| (reading.clone(), Tone::Gloss))
            .collect();
        Self {
            index: (position + 1).to_string(),
            text: candidate.text.clone(),
            annotation,
            cloud: false,
        }
    }
}
