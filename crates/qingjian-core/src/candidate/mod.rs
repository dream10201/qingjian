//! 候选词数据模型。

mod kind;
mod layout;
mod list;

use serde::{Deserialize, Serialize};

pub use kind::CandidateKind;
pub use layout::{CandidateLayout, Cell};
pub use list::CandidateList;

/// 一个可上屏的候选。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candidate {
    /// 上屏文本。
    pub text: String,

    /// 来源类型。
    pub kind: CandidateKind,

    /// 该候选对应的拼音音节，供平台层高亮已匹配部分。
    pub syllables: Vec<String>,

    /// 附注（emoji 对应的词、问字答案的读音），中文候选为 `None`。
    pub reading: Option<String>,
}
