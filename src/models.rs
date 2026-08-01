use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnneagramType {
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
    Type6,
    Type7,
    Type8,
    Type9,
}

impl EnneagramType {
    pub const ALL: [Self; 9] = [
        Self::Type1,
        Self::Type2,
        Self::Type3,
        Self::Type4,
        Self::Type5,
        Self::Type6,
        Self::Type7,
        Self::Type8,
        Self::Type9,
    ];

    pub fn index(self) -> usize {
        match self {
            Self::Type1 => 0,
            Self::Type2 => 1,
            Self::Type3 => 2,
            Self::Type4 => 3,
            Self::Type5 => 4,
            Self::Type6 => 5,
            Self::Type7 => 6,
            Self::Type8 => 7,
            Self::Type9 => 8,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Type1 => "タイプ1",
            Self::Type2 => "タイプ2",
            Self::Type3 => "タイプ3",
            Self::Type4 => "タイプ4",
            Self::Type5 => "タイプ5",
            Self::Type6 => "タイプ6",
            Self::Type7 => "タイプ7",
            Self::Type8 => "タイプ8",
            Self::Type9 => "タイプ9",
        }
    }

    pub fn profile_placeholder(self) -> &'static str {
        match self {
            Self::Type1 => "長所: 誠実さと改善志向。短所: 完璧を求めすぎる傾向。対人傾向: 筋道を大切にして相手を支える。",
            Self::Type2 => "長所: 思いやりと献身性。短所: 過剰な自己犠牲。対人傾向: 人を助けながら関係を深める。",
            Self::Type3 => "長所: 目標達成力と実行力。短所: 評価への過敏さ。対人傾向: 成果を通じて信頼を築く。",
            Self::Type4 => "長所: 感受性と独自性。短所: 気分の波に左右されやすい。対人傾向: 深い共感を重視する。",
            Self::Type5 => "長所: 分析力と洞察力。短所: 距離を取りすぎる傾向。対人傾向: 観察しながら慎重に関わる。",
            Self::Type6 => "長所: 責任感と備えの強さ。短所: 不安を抱え込みやすい。対人傾向: 信頼関係を丁寧に確かめる。",
            Self::Type7 => "長所: 発想力と前向きさ。短所: 注意が散りやすい。対人傾向: 明るさで場を活性化する。",
            Self::Type8 => "長所: 決断力と推進力。短所: 強く出すぎることがある。対人傾向: 率直で主導的に関わる。",
            Self::Type9 => "長所: 包容力と調整力。短所: 自己主張を後回しにしがち。対人傾向: 調和を保ちながら周囲をつなぐ。",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisId {
    StrengthPreference,
    UnconsciousHabit,
    CommunicationStyle,
    RelationshipStyle,
    CommonPattern,
    AuthenticTrait,
    ExternalImpression,
}

impl AxisId {
    pub const ALL: [Self; 7] = [
        Self::StrengthPreference,
        Self::UnconsciousHabit,
        Self::CommunicationStyle,
        Self::RelationshipStyle,
        Self::CommonPattern,
        Self::AuthenticTrait,
        Self::ExternalImpression,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::StrengthPreference => "自分のこだわり(良いところ)",
            Self::UnconsciousHabit => "無意識のクセ(良くないところ)",
            Self::CommunicationStyle => "コミュニケーションの傾向",
            Self::RelationshipStyle => "対人関係の傾向",
            Self::CommonPattern => "よくあること",
            Self::AuthenticTrait => "自分らしいこと",
            Self::ExternalImpression => "他者から見える印象",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuestionItem {
    pub enneagram_type: EnneagramType,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AxisQuestion {
    pub axis: AxisId,
    pub items: Vec<QuestionItem>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
    Most,
    Next,
    Slight,
}

impl Rank {
    pub fn label(self) -> &'static str {
        match self {
            Rank::Most => "最も当てはまる",
            Rank::Next => "次に当てはまる",
            Rank::Slight => "少し当てはまる",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AxisSelection {
    pub most: Option<usize>,
    pub next: Option<usize>,
    pub slight: Vec<usize>,
}

impl AxisSelection {
    pub fn rank_of(&self, item_index: usize) -> Option<Rank> {
        if self.most == Some(item_index) {
            return Some(Rank::Most);
        }
        if self.next == Some(item_index) {
            return Some(Rank::Next);
        }
        if self.slight.contains(&item_index) {
            return Some(Rank::Slight);
        }
        None
    }

    pub fn is_complete(&self) -> bool {
        self.most.is_some() && self.next.is_some() && !self.slight.is_empty()
    }

    pub fn score_for_item(&self, item_index: usize) -> u32 {
        if self.most == Some(item_index) {
            return 5;
        }
        if self.next == Some(item_index) {
            return 4;
        }
        if self.slight.contains(&item_index) {
            return 2;
        }
        0
    }

    pub fn assign_rank(&mut self, rank: Rank, item_index: usize) -> Result<(), &'static str> {
        self.clear_item(item_index);

        match rank {
            Rank::Most => {
                self.most = Some(item_index);
            }
            Rank::Next => {
                self.next = Some(item_index);
            }
            Rank::Slight => {
                if self.slight.len() >= 2 {
                    return Err("「少し当てはまる」は最大2個までです。");
                }
                self.slight.push(item_index);
            }
        }

        Ok(())
    }

    pub fn clear_item(&mut self, item_index: usize) {
        if self.most == Some(item_index) {
            self.most = None;
        }
        if self.next == Some(item_index) {
            self.next = None;
        }
        self.slight.retain(|v| *v != item_index);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosisResult {
    pub scores: [u32; 9],
    pub top_types: Vec<EnneagramType>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoredResult {
    pub scores: [u32; 9],
    pub top_types: Vec<EnneagramType>,
    pub generated_at: String,
}
