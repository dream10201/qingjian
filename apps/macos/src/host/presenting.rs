//! 呈现：删候选、按应用关英文候选、会话重置与候选窗口绘制。

use super::*;

impl Host {
    /// 删掉当前页第 `offset` 格的候选：用户词整个删、词库词清学习。返回给用户看的一句话；那格没有候选返回 `None`。
    pub fn forget_candidate(&mut self, offset: usize) -> Option<String> {
        let index = self.session.index_on_page(offset)?;
        let candidate = self.session.candidate(index)?;
        let forgotten = self.engine.forget(&candidate);
        let text = &candidate.text;
        Some(if forgotten.user_word {
            format!("已删除用户词「{text}」")
        } else if forgotten.learning {
            format!("已忘掉对「{text}」的学习记录")
        } else {
            format!("「{text}」是词库里的词，也没有学习记录，没什么可删")
        })
    }

    /// 这个应用里英文模式给不给候选：全局开关开着，且应用不在 `[apps] english_candidates_off` 里。
    pub fn english_candidates_in(&self, bundle: Option<&str>) -> bool {
        self.english_candidates && !bundle.is_some_and(|b| self.apps.english_candidates_off(b))
    }

    /// 新一轮候选：每页格数取配置与窗口能画的行数中较小者，云端槽位数取配置。
    pub fn reset_session(&mut self, preedit: Option<Preedit>, candidates: Vec<Candidate>) {
        self.status = None;
        let page_size = self.page_size.min(self.window.max_rows()).max(1);
        self.session
            .reset(preedit, candidates, page_size, self.cloud_slots);
    }

    /// 按会话状态画候选窗口。候选为空且没有 preedit 时收窗。
    pub fn render(&mut self) {
        let size = self.session.layout.page_size();
        let page = self.session.page;
        let rows: Vec<Row> = self
            .session
            .page_cells()
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                let candidate = cell.candidate();
                let mut row = Row::from_candidate(i, candidate);
                row.cloud = candidate.kind == CandidateKind::Cloud;
                row
            })
            .collect();
        // 配置成只在行内显示时，窗口顶部不画拼音行
        let preedit = self
            .preedit_mode
            .in_window()
            .then(|| self.session.preedit.clone())
            .flatten();
        if rows.is_empty() && self.session.preedit.is_none() {
            self.window.hide();
            return;
        }
        let pages = self.session.pages();
        let footer = (pages > 1).then(|| format!("{}/{pages}", page + 1));
        let frame = Frame {
            preedit,
            rows,
            highlighted: self.session.highlighted.saturating_sub(page * size),
            footer,
            sentence: self.sentence.clone(),
            status: self.status.clone(),
        };
        self.window.show(frame, self.anchor);
    }
}
